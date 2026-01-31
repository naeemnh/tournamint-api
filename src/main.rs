use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;

// ==================== NEW DDD ARCHITECTURE ====================
mod application;
mod domain;
mod infra;
mod shared;

// ==================== LEGACY MODULES (for gradual migration) ====================
mod config;
mod constants;
mod controllers;
mod formatters;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app_config = shared::AppConfig::from_env();
    let bind_address = app_config.bind_address();

    let pool = infra::db::DbConfig::create_db_pool()
        .await
        .expect("Failed to create pool");

    let user_repo = Arc::new(infra::db::PgUserRepository::new(pool.clone()));
    let token_repo = Arc::new(infra::db::PgTokenRepository::new(pool.clone()));
    let profile_repo = Arc::new(infra::db::PgUserProfileRepository::new(pool.clone()));
    let auth_use_cases = Arc::new(application::AuthUseCases::new(
        Arc::clone(&user_repo),
        Arc::clone(&token_repo),
    ));
    let user_use_cases = Arc::new(application::UserUseCases::new(
        Arc::clone(&user_repo),
        Arc::clone(&profile_repo),
    ));

    let player_repo = Arc::new(infra::db::PgPlayerRepository::new(pool.clone()));
    let team_repo = Arc::new(infra::db::PgTeamRepository::new(pool.clone()));
    let team_member_repo = Arc::new(infra::db::PgTeamMemberRepository::new(pool.clone()));
    let participant_use_cases = Arc::new(application::ParticipantUseCases::new(
        Arc::clone(&player_repo),
        Arc::clone(&team_repo),
        Arc::clone(&team_member_repo),
    ));

    let tournament_repo = Arc::new(infra::db::PgTournamentRepository::new(pool.clone()));
    let tournament_category_repo =
        Arc::new(infra::db::PgTournamentCategoryRepository::new(pool.clone()));
    let tournament_registration_repo =
        Arc::new(infra::db::PgTournamentRegistrationRepository::new(pool.clone()));
    let tournament_bracket_repo =
        Arc::new(infra::db::PgTournamentBracketRepository::new(pool.clone()));
    let tournament_standings_repo =
        Arc::new(infra::db::PgTournamentStandingsRepository::new(pool.clone()));
    let tournament_use_cases = Arc::new(application::TournamentUseCases::new(
        Arc::clone(&tournament_repo),
        Arc::clone(&tournament_category_repo),
        Arc::clone(&tournament_registration_repo),
        Arc::clone(&tournament_bracket_repo),
        Arc::clone(&tournament_standings_repo),
    ));

    let match_repo = Arc::new(infra::db::PgMatchRepository::new(pool.clone()));
    let match_result_repo = Arc::new(infra::db::PgMatchResultRepository::new(pool.clone()));
    let match_use_cases = Arc::new(application::MatchUseCases::new(
        Arc::clone(&match_repo),
        Arc::clone(&match_result_repo),
    ));

    let notification_repo = Arc::new(infra::db::PgNotificationRepository::new(pool.clone()));
    let notification_use_cases =
        Arc::new(application::NotificationUseCases::new(Arc::clone(&notification_repo)));

    let payment_repo = Arc::new(infra::db::PgPaymentRepository::new(pool.clone()));
    let payment_use_cases = Arc::new(application::PaymentUseCases::new(Arc::clone(&payment_repo)));

    let statistics_repo = Arc::new(infra::db::PgStatisticsRepository::new(pool.clone()));
    let statistics_use_cases = Arc::new(application::StatisticsUseCases::new(
        Arc::clone(&statistics_repo),
        Arc::clone(&player_repo),
    ));

    println!("Starting server at http://{}", &bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(infra::api::middleware::AuthMiddleware)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Arc::clone(&auth_use_cases)))
            .app_data(web::Data::new(Arc::clone(&user_use_cases)))
            .app_data(web::Data::new(Arc::clone(&participant_use_cases)))
            .app_data(web::Data::new(Arc::clone(&tournament_use_cases)))
            .app_data(web::Data::new(Arc::clone(&match_use_cases)))
            .app_data(web::Data::new(Arc::clone(&notification_use_cases)))
            .app_data(web::Data::new(Arc::clone(&payment_use_cases)))
            .app_data(web::Data::new(Arc::clone(&statistics_use_cases)))
            .configure(infra::api::api_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
