use actix_web::web;

use crate::controllers::TournamentController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/tournaments")
            // Basic CRUD
            .route("", web::post().to(TournamentController::create))
            .route("", web::get().to(TournamentController::get_all))
            .route("/{id}", web::get().to(TournamentController::get_by_id))
            .route("/{id}", web::put().to(TournamentController::update))
            .route("/{id}", web::delete().to(TournamentController::delete))
            
            // Search and filtering
            .route("/search", web::get().to(TournamentController::search))
            .route("/my", web::get().to(TournamentController::get_my_tournaments))
            .route("/featured", web::get().to(TournamentController::get_featured))
            .route("/upcoming", web::get().to(TournamentController::get_upcoming))
            .route("/status/{status}", web::get().to(TournamentController::get_by_status))
            
            // Tournament management
            .route("/{id}/publish", web::put().to(TournamentController::publish))
            .route("/{id}/start", web::put().to(TournamentController::start))
            .route("/{id}/complete", web::put().to(TournamentController::complete))
            .route("/{id}/cancel", web::put().to(TournamentController::cancel))
            
            // Statistics and participants
            .route("/{id}/stats", web::get().to(TournamentController::get_stats))
            .route("/{id}/participants", web::get().to(TournamentController::get_participants))
            
            // Export and duplicate
            .route("/{id}/export", web::get().to(TournamentController::export))
            .route("/{id}/duplicate", web::post().to(TournamentController::duplicate))
            
            // Templates
            .route("/templates", web::get().to(TournamentController::get_templates))
            .route("/templates/{template_id}", web::post().to(TournamentController::create_from_template))
            
            // Organizer functions
            .route("/{id}/dashboard", web::get().to(TournamentController::get_dashboard))
            .route("/{id}/settings", web::put().to(TournamentController::update_settings))
            
            // Categories (sub-resource)
            .route("/{id}/categories", web::get().to(TournamentController::get_categories))
            
            // Registrations (sub-resource)
            .route("/{id}/registrations", web::get().to(TournamentController::get_registrations))
    );
}