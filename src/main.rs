use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod config;
mod constants;
mod controllers;
mod formatters;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_config = config::AppConfig::from_env();
    let bind_address = app_config.bind_address();

    let pool = config::DbConfig::create_db_pool()
        .await
        .expect("Failed to create pool");

    println!("Starting server at http://{}", &bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auth::AuthMiddleware)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::api_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
