use actix_web::HttpResponse;

use crate::shared::ApiResponse;

/// Notification handlers - placeholder
pub struct NotificationHandler;

impl NotificationHandler {
    pub async fn placeholder() -> HttpResponse {
        ApiResponse::error("Notification handlers not implemented - migrate from controllers/notification_controller.rs")
    }
}
