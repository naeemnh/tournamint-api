use actix_web::web::ServiceConfig;

mod auth;
mod match_result_routes;
mod match_routes;
mod notification_routes;
mod payment_routes;
mod player;
mod statistics_routes;
mod team;
mod team_member;
mod tournament;
mod tournament_bracket_routes;
mod tournament_category;
mod tournament_registration;
mod tournament_standings_routes;
mod user;
mod user_profile_routes;

pub fn api_routes(app: &mut ServiceConfig) {
    auth::routes(app);
    user::routes(app);
    user_profile_routes::routes(app);
    team::routes(app);
    player::routes(app);
    team_member::routes(app);
    tournament::routes(app);
    tournament_category::routes(app);
    tournament_registration::routes(app);
    match_routes::routes(app);
    match_result_routes::routes(app);
    tournament_bracket_routes::routes(app);
    tournament_standings_routes::routes(app);
    notification_routes::routes(app);
    payment_routes::routes(app);
    statistics_routes::routes(app);
    statistics_routes::analytics_routes(app);
}
