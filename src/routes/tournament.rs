use actix_web::web;

use crate::controllers::TournamentController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/tournaments")
            .route("", web::post().to(TournamentController::create))
            .route("", web::get().to(TournamentController::get_all))
            .route("/{id}", web::get().to(TournamentController::get_by_id))
            .route("/status/{status}", web::get().to(TournamentController::get_by_status))
            .route("/{id}", web::put().to(TournamentController::update))
            .route("/{id}", web::delete().to(TournamentController::delete))
    );
}