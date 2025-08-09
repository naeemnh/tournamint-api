use actix_web::web;

use crate::controllers::TournamentStandingsController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/standings")
            .route(
                "/tournament/{tournament_id}",
                web::get().to(TournamentStandingsController::get_tournament_standings),
            )
            .route(
                "/category/{category_id}",
                web::get().to(TournamentStandingsController::get_category_standings),
            )
            .route(
                "/update/{tournament_id}",
                web::put().to(TournamentStandingsController::update_standings),
            ),
    );
}