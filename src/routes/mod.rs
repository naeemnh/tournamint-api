use actix_web::web::ServiceConfig;

mod user;

pub fn api_routes(app: &mut ServiceConfig) {
    user::routes(app);
}
