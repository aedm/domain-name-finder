mod database_reader;

use crate::database_reader::Database;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use itertools::Itertools;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// This struct represents state
struct AppState {
    app_name: String,
    database: RwLock<Arc<Database>>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let database = data.database.read().unwrap().clone();
    let words = database.iter().join(",");
    HttpResponse::Ok().body(format!(
        "words: {}, thread: {:?}\n",
        words,
        thread::current().id()
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn update_database(app_state: web::Data<AppState>) {
    let mut counter = 0;
    loop {
        actix_web::rt::time::delay_for(Duration::from_millis(1000)).await;

        counter += 1;
        println!("counter: {}", counter);
        let heavy: i64 = (0i64..100_000_000).map(|x| (x + counter) % 2).sum();
        println!("heavy: {}", heavy);

        let mut new_db = Database::new();
        new_db.insert(format!("counter {}", counter));
        new_db.insert("lézer".into());
        let mut db_ref = app_state.database.write().unwrap();
        *db_ref = Arc::new(new_db);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Start");
    let mut database = Database::new();
    database.insert("foo".into());
    database.insert("bar".into());

    let app_state = web::Data::new(AppState {
        app_name: String::from("Actix-web"),
        database: RwLock::new(Arc::new(database)),
    });

    let app_state_clone = app_state.clone();
    actix_web::rt::spawn(async move {
        update_database(app_state_clone).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
