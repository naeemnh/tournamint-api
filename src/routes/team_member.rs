use actix_web::web;

use crate::controllers::TeamMemberController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/team_members")
            .route("", web::post().to(TeamMemberController::post))
            .route(
                "/team/{id}",
                web::get().to(TeamMemberController::get_by_team),
            )
            .route(
                "/{team_id}/{player_id}",
                web::get().to(TeamMemberController::get_by_id),
            )
            .route(
                "/{team_id}/{player_id}",
                web::put().to(TeamMemberController::update),
            )
            .route(
                "/{team_id}/{player_id}",
                web::delete().to(TeamMemberController::delete),
            ),
    );
}
