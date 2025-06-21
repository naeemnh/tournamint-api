use actix_web::web;

use crate::controllers::TournamentRegistrationController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/tournament_registrations")
            .route("", web::post().to(TournamentRegistrationController::create))
            .route("/{id}", web::get().to(TournamentRegistrationController::get_by_id))
            .route("/{id}", web::put().to(TournamentRegistrationController::update))
            .route("/{id}", web::delete().to(TournamentRegistrationController::delete))
            .route(
                "/category/{category_id}",
                web::get().to(TournamentRegistrationController::get_by_tournament_category),
            )
            .route(
                "/tournament/{tournament_id}",
                web::get().to(TournamentRegistrationController::get_by_tournament),
            )
            .route(
                "/player/{player_id}",
                web::get().to(TournamentRegistrationController::get_by_player),
            )
            .route(
                "/team/{team_id}",
                web::get().to(TournamentRegistrationController::get_by_team),
            ),
    );
}