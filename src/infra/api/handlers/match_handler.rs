use actix_web::HttpResponse;

use crate::shared::ApiResponse;

/// Match handlers - placeholder
pub struct MatchHandler;

impl MatchHandler {
    pub async fn placeholder() -> HttpResponse {
        ApiResponse::error("Match handlers not implemented - migrate from controllers/match_controller.rs")
    }
}

/// Match result handlers - placeholder
pub struct MatchResultHandler;

impl MatchResultHandler {
    pub async fn placeholder() -> HttpResponse {
        ApiResponse::error("Match result handlers not implemented - migrate from controllers/match_result_controller.rs")
    }
}
