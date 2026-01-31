use actix_web::HttpResponse;

use crate::shared::ApiResponse;

/// Statistics handlers - placeholder
pub struct StatisticsHandler;

impl StatisticsHandler {
    pub async fn placeholder() -> HttpResponse {
        ApiResponse::error("Statistics handlers not implemented - migrate from controllers/statistics_controller.rs")
    }
}

/// Analytics handlers - placeholder
pub struct AnalyticsHandler;

impl AnalyticsHandler {
    pub async fn placeholder() -> HttpResponse {
        ApiResponse::error("Analytics handlers not implemented - migrate from controllers/statistics_controller.rs")
    }
}
