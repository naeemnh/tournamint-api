use actix_web::web;

use crate::controllers::notification_controller::NotificationController;
use crate::middlewares::auth::auth_middleware;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .wrap(auth_middleware())
        .route("", web::get().to(NotificationController::index))
        .route("/unread", web::get().to(NotificationController::get_unread))
        .route("/count", web::get().to(NotificationController::get_unread_count))
        .route("/read-all", web::put().to(NotificationController::mark_all_as_read))
        .route("/send", web::post().to(NotificationController::send_notification))
        .route("/{id}/read", web::put().to(NotificationController::mark_as_read))
        .route("/{id}", web::delete().to(NotificationController::delete))
    );
}
