use crate::services::auth_service;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct GoogleCallbackQuery {
    code: String,
}

pub async fn start_google_login() -> impl Responder {
    let auth_url = auth_service::google_login(
        std::env::var("APP_ENV")
            .expect("Application environment (APP_ENV) type is not defined.")
            .as_str(),
    );

    // Redirect to Google's authorization page
    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

pub async fn google_callback(
    pool: web::Data<PgPool>,
    code: web::Query<GoogleCallbackQuery>,
) -> impl Responder {
    auth_service::google_callback(&pool, &code.code).await
}
