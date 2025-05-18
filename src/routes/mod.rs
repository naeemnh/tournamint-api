use actix_web::web::ServiceConfig;

mod player;
mod team;
mod team_member;
mod user;

pub fn api_routes(app: &mut ServiceConfig) {
    user::routes(app);
    team::routes(app);
    player::routes(app);
    team_member::routes(app);
}
