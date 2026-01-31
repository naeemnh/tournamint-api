use actix_web::HttpResponse;

use crate::shared::ApiResponse;

/// Payment handlers - placeholder
pub struct PaymentHandler;

impl PaymentHandler {
    pub async fn placeholder() -> HttpResponse {
        ApiResponse::error("Payment handlers not implemented - migrate from controllers/payment_controller.rs")
    }
}
