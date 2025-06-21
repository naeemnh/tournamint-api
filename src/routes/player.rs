use actix_web::web;

use crate::controllers::PlayerController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/players")
            .route("", web::get().to(PlayerController::index))
            .route("", web::post().to(PlayerController::post))
            .route("/{id}", web::get().to(PlayerController::show))
            .route("/{id}", web::post().to(PlayerController::update))
            .route("/{id}", web::delete().to(PlayerController::delete)),
    );
}
