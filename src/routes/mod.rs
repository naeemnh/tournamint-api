use actix_web::web::ServiceConfig;

mod auth;
mod player;
mod team;
mod team_member;
mod tournament;
mod user;

pub fn api_routes(app: &mut ServiceConfig) {
    auth::routes(app);
    user::routes(app);
    team::routes(app);
    player::routes(app);
    tournament::routes(app);
    team_member::routes(app);
}
