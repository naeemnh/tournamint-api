use actix_web::web;

use crate::controllers::TeamController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/teams")
            .route("", web::get().to(TeamController::index))
            .route("", web::post().to(TeamController::post))
            .route("/{id}", web::get().to(TeamController::show))
            .route("/{id}", web::post().to(TeamController::update))
            .route("/{id}", web::delete().to(TeamController::delete)),
    );
}
