mod database;
mod database_reader;
mod search;

use crate::database::Database;
use crate::database_reader::read_database;
use crate::search::{batch_lookup, search, BatchLookupInput, SearchInput, SearchResult};

use actix_web::web::Data;
use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use peak_alloc::PeakAlloc;
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};

// This struct represents state
struct AppState {
    database: Option<Database>,
}

impl AppState {
    fn get_database(&self) -> Result<&Database> {
        self.database
            .as_ref()
            .ok_or(error::ErrorInternalServerError("Database not loaded"))
    }
}

#[post("/api/search")]
async fn search_endpoint(
    search_input: web::Json<SearchInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    let database = data.database.as_ref().unwrap();
    let result = search(database, &search_input);
    web::Json(result)
}

#[post("/api/batch-lookup")]
async fn batch_lookup_endpoint(
    input: web::Json<BatchLookupInput>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let database = data.get_database()?;
    Result::Ok(web::Json(batch_lookup(database, input.into_inner())))
}

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

// #[actix_web::main]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Start");

    let is_dev_server = std::env::var("DEV_SERVER").is_ok();

    let database = read_database().await?;
    println!("Memory usage: {:.1} MB", PEAK_ALLOC.current_usage_as_mb());

    let app_state = AppState {
        database: Some(database),
        // database: None,
    };
    let app_state_wrapped = web::Data::new(app_state);

    let port = if is_dev_server { "8000" } else { "9000" };
    let address = format!("0.0.0.0:{port}");
    println!("Serving on {}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state_wrapped.clone())
            .service(search_endpoint)
            .service(batch_lookup_endpoint)
    })
    .bind(address)?
    .run()
    .await?;

    println!("End");

    Ok(())
}
