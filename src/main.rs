use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod config;
mod constants;
mod controllers;
mod formatters;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = config::db::create_db_pool()
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::api_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
