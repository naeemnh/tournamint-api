use actix_web::web;

use crate::controllers::TournamentBracketController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/brackets")
            .route(
                "/tournament/{tournament_id}",
                web::get().to(TournamentBracketController::get_tournament_bracket),
            )
            .route(
                "/category/{category_id}",
                web::get().to(TournamentBracketController::get_category_bracket),
            )
            .route(
                "/generate/{tournament_id}",
                web::put().to(TournamentBracketController::generate_bracket),
            ),
    );
}