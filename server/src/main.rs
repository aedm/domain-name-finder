mod database;
mod database_reader;
mod search;

use crate::database::Database;
use crate::database_reader::read_database;
use crate::search::{batch_lookup, search, BatchLookupInput, SearchInput, SearchResult};

use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use itertools::Itertools;
use peak_alloc::PeakAlloc;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::{io, thread};

// This struct represents state
struct AppState {
    app_name: String,
    database: Arc<Database>,
}

#[post("/api/search")]
async fn search_endpoint(
    search_input: web::Json<SearchInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = search(&data.database, &search_input);
    web::Json(result)
}

#[post("/api/batch-lookup")]
async fn batch_lookup_endpoint(
    input: web::Json<BatchLookupInput>,
    data: web::Data<AppState>,
) -> impl Responder {
    web::Json(batch_lookup(&data.database, input.into_inner()))
}

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

// #[actix_web::main]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Start");

    let mut database = read_database().await?;

    println!("Memory usage: {:.1} MB", PEAK_ALLOC.current_usage_as_mb());

    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix-web"),
        database: Arc::new(database),
    });

    // let app_state_clone = app_state.clone();
    // actix_web::rt::spawn(async move {
    //     update_database(app_state_clone).await;
    // });
    //
    let address = "0.0.0.0:8080";
    println!("Serving on {}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(search_endpoint)
    })
    .bind(address)?
    .run()
    .await?;

    println!("End");

    Ok(())
}
