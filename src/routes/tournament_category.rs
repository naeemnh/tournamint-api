use actix_web::web;

use crate::controllers::TournamentCategoryController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/tournament_categories")
            .route("", web::post().to(TournamentCategoryController::create))
            .route("/tournament/{tournament_id}", web::get().to(TournamentCategoryController::get_by_tournament))
            .route("/{id}", web::get().to(TournamentCategoryController::get_by_id))
            .route("/{id}", web::put().to(TournamentCategoryController::update))
            .route("/{id}", web::delete().to(TournamentCategoryController::delete))
    );
}