use actix_web::HttpResponse;

use crate::shared::ApiResponse;

pub struct HealthHandler;

impl HealthHandler {
    pub async fn health() -> HttpResponse {
        ApiResponse::success("OK", Some(serde_json::json!({})))
    }
}
