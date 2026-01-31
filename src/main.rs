use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;

// ==================== NEW DDD ARCHITECTURE ====================
mod application;
mod domain;
mod infra;
mod shared;

// ==================== LEGACY MODULES (for gradual migration) ====================
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

    let app_config = shared::AppConfig::from_env();
    let bind_address = app_config.bind_address();

    let pool = infra::db::DbConfig::create_db_pool()
        .await
        .expect("Failed to create pool");

    let user_repo = Arc::new(infra::db::PgUserRepository::new(pool.clone()));
    let token_repo = Arc::new(infra::db::PgTokenRepository::new(pool.clone()));
    let auth_use_cases = Arc::new(application::AuthUseCases::new(user_repo, token_repo));

    println!("Starting server at http://{}", &bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(infra::api::middleware::AuthMiddleware)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Arc::clone(&auth_use_cases)))
            .configure(infra::api::api_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
