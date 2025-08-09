use actix_web::web;
use crate::controllers::match_controller::MatchController;
use crate::middlewares::auth::AuthMiddleware;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/matches")
            .wrap(AuthMiddleware)
            // Basic CRUD
            .route("", web::post().to(MatchController::create_match))
            .route("/{id}", web::get().to(MatchController::get_match))
            .route("/{id}", web::put().to(MatchController::update_match))
            .route("/{id}", web::delete().to(MatchController::delete_match))
            
            // Query by tournament/category
            .route("/tournament/{tournament_id}", web::get().to(MatchController::get_matches_by_tournament))
            .route("/category/{category_id}", web::get().to(MatchController::get_matches_by_category))
            
            // Match with participants
            .route("/{id}/participants", web::get().to(MatchController::get_match_with_participants))
            
            // Match status management
            .route("/{id}/status", web::put().to(MatchController::update_match_status))
            .route("/{id}/start", web::put().to(MatchController::start_match))
            .route("/{id}/complete", web::put().to(MatchController::complete_match))
            .route("/{id}/cancel", web::put().to(MatchController::cancel_match))
            .route("/{id}/postpone", web::put().to(MatchController::postpone_match))
            
            // Scheduling
            .route("/schedule", web::get().to(MatchController::get_match_schedule))
            .route("/my/upcoming", web::get().to(MatchController::get_my_upcoming_matches))
            .route("/my/history", web::get().to(MatchController::get_my_match_history))
            .route("/{id}/reschedule", web::put().to(MatchController::reschedule_match))
            
            // Live matches
            .route("/live", web::get().to(MatchController::get_live_matches))
            .route("/{id}/live", web::put().to(MatchController::update_match_live))
            
            // Analytics and statistics
            .route("/{id}/analytics", web::get().to(MatchController::get_match_analytics))
            .route("/{id}/statistics", web::get().to(MatchController::get_match_statistics))
            
            // Media and files
            .route("/{id}/media", web::get().to(MatchController::get_match_media))
            .route("/{id}/video", web::post().to(MatchController::upload_match_video))
            .route("/{id}/photo", web::post().to(MatchController::upload_match_photo))
            
            // Comments
            .route("/{id}/comments", web::get().to(MatchController::get_match_comments))
            .route("/{id}/comments", web::post().to(MatchController::add_match_comment))
            
            // Subscriptions
            .route("/{id}/subscribe", web::post().to(MatchController::subscribe_to_match))
            .route("/{id}/subscribe", web::delete().to(MatchController::unsubscribe_from_match))
            
            // Bulk operations
            .route("/bulk/update", web::put().to(MatchController::bulk_update_matches))
            .route("/bulk/cancel", web::put().to(MatchController::bulk_cancel_matches))
    );
}