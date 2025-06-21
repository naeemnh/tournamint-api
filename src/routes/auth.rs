use crate::controllers::AuthController;
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/auth/google")
            .route("", web::get().to(AuthController::start_google_login))
            .route("/callback", web::get().to(AuthController::google_callback)),
    );
}
