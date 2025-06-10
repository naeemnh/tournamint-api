use actix_web::{web, App, HttpServer};

mod config;
mod constants;
mod controllers;
mod formatters;
mod middlewares;
mod models;
mod providers;
mod repositories;
mod routes;
mod services;
mod state;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = providers::DbProvider::pool().await;

    let app_url = config::AppConfig::get_app_url();

    HttpServer::new(move || {
        App::new()
            .wrap(middlewares::auth::AuthMiddleware)
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::api_routes)
    })
    .bind(app_url)?
    .run()
    .await
}
