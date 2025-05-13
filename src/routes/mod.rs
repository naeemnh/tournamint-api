use actix_web::web::ServiceConfig;

mod player;
mod user;

pub fn api_routes(app: &mut ServiceConfig) {
    user::routes(app);
    player::routes(app);
}
