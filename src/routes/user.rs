use actix_web::web;

use crate::controllers::user_controller;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/users")
            .route("", web::get().to(user_controller::index))
            .route("", web::post().to(user_controller::post))
            .route("/{id}", web::get().to(user_controller::show))
            .route("/{id}", web::post().to(user_controller::update))
            .route("/{id}", web::delete().to(user_controller::delete)),
    );
}
