mod database;
mod database_reader;
mod search;

use crate::database::Database;
use crate::database_reader::read_database;
use crate::search::{batch_lookup, search, BatchLookupInput, SearchInput, SearchResult};
use std::time::Instant;

use actix_web::{error, post, web, App, HttpResponse, HttpServer, Responder, Result};
use peak_alloc::PeakAlloc;

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
async fn api_search(
    search_input: web::Json<SearchInput>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    // println!("/api/search {search_input:?}");
    let now = Instant::now();
    let database = data.database.as_ref();
    let result = search(database, &search_input)
        .await
        .map_err(|err| error::ErrorInternalServerError(err.to_string()))?;
    println!("Response time {}", now.elapsed().as_millis());
    Ok(web::Json(result))
}

#[post("/api/batch-lookup")]
async fn api_batch_lookup(
    input: web::Json<BatchLookupInput>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let database = data.get_database()?;
    let result = batch_lookup(database, input.into_inner());
    Result::Ok(web::Json(result))
}

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

// #[actix_web::main]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Start");

    let is_dev_server = std::env::var("DEV_SERVER").is_ok();

    let database = if is_dev_server {
        None
    } else {
        Some(read_database().await?)
    };
    println!("Memory usage: {:.1} MB", PEAK_ALLOC.current_usage_as_mb());

    let app_state = AppState {
        database,
        // database: None,
    };
    let app_state_wrapped = web::Data::new(app_state);

    let port = if is_dev_server { "8000" } else { "9000" };
    let address = format!("0.0.0.0:{port}");
    println!("Serving on {}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state_wrapped.clone())
            .service(api_search)
            .service(api_batch_lookup)
    })
    .bind(address)?
    .run()
    .await?;

    println!("End");

    Ok(())
}
