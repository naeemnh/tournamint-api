use actix_web::web;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::scope("tournaments"));
}
