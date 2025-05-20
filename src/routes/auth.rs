use crate::controllers::auth_controller::{self, start_google_login};
use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/auth/google")
            .route("", web::get().to(start_google_login))
            .route("/callback", web::get().to(auth_controller::google_callback)),
    );
}
