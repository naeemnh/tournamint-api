use actix_web::web;

use crate::controllers::team_controller;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/teams")
            .route("", web::get().to(team_controller::index))
            .route("", web::post().to(team_controller::post))
            .route("/{id}", web::get().to(team_controller::show))
            .route("/{id}", web::post().to(team_controller::update))
            .route("/{id}", web::delete().to(team_controller::delete)),
    );
}
