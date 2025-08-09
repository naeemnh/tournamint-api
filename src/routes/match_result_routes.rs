use actix_web::web;
use crate::controllers::match_result_controller::MatchResultController;
use crate::middlewares::auth::AuthMiddleware;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/match-results")
            .wrap(AuthMiddleware)
            // Core CRUD operations
            .route("", web::post().to(MatchResultController::create_match_result))
            .route("/{id}", web::get().to(MatchResultController::get_match_result))
            .route("/{id}", web::put().to(MatchResultController::update_match_result))
            .route("/{id}", web::delete().to(MatchResultController::delete_match_result))
            
            // Match-specific operations (as requested)
            .route("/match/{match_id}", web::get().to(MatchResultController::get_match_results_by_match))
            .route("/match/{match_id}/summary", web::get().to(MatchResultController::get_match_score_summary))
            .route("/match/{match_id}/count", web::get().to(MatchResultController::count_match_results))
            .route("/match/{match_id}/validate", web::get().to(MatchResultController::validate_match_scores))
            .route("/match/{match_id}", web::delete().to(MatchResultController::delete_all_match_results))
            
            // Set-specific operations
            .route("/match/{match_id}/set/{set_number}", web::get().to(MatchResultController::get_match_results_by_set))
            
            // Bulk operations
            .route("/bulk", web::post().to(MatchResultController::bulk_create_match_results))
    );
}