mod controller;

use actix_web::{web, App, HttpServer};
use chrono::{DateTime, Utc};
use controller::transaction::transaction_config;
use moka::future::Cache;
use std::time::Duration;

pub struct AppData {
    cache: Cache<DateTime<Utc>, f32>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cache: Cache<DateTime<Utc>, f32> = Cache::builder()
        .time_to_live(Duration::from_secs(10))
        .build();
    let app_data = web::Data::new(AppData { cache });
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/api").configure(transaction_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
