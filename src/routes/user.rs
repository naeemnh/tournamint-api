use actix_web::web;

use crate::controllers::UserController;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/users")
            .route("", web::get().to(UserController::index))
            .route("", web::post().to(UserController::post))
            .route("/{id}", web::get().to(UserController::show))
            .route("/{id}", web::post().to(UserController::update))
            .route("/{id}", web::delete().to(UserController::delete)),
    );
}
