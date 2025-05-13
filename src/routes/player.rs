use actix_web::web;

use crate::controllers::player_controller;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/players")
            .route("", web::get().to(player_controller::index))
            .route("", web::post().to(player_controller::post))
            .route("/{id}", web::get().to(player_controller::show))
            .route("/{id}", web::post().to(player_controller::update))
            .route("/{id}", web::delete().to(player_controller::delete)),
    );
}
