use actix_web::web;

use crate::controllers::UserProfileController;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .route("", web::get().to(UserProfileController::get_current_profile))
            .route("", web::put().to(UserProfileController::update_current_profile))
            .route(
                "/preferences",
                web::post().to(UserProfileController::update_preferences),
            )
            .route(
                "/notifications",
                web::post().to(UserProfileController::update_notification_preferences),
            )
            .route(
                "/privacy",
                web::post().to(UserProfileController::update_privacy_settings),
            )
            .route("/avatar", web::post().to(UserProfileController::update_avatar))
            .route("/avatar", web::delete().to(UserProfileController::remove_avatar))
            .route("/{user_id}", web::get().to(UserProfileController::get_public_profile)),
    );
}