use actix_web::web;

use super::handlers::{
    auth_handler::AuthHandler,
    user_handler::{UserHandler, UserProfileHandler},
    tournament_handler::TournamentHandler,
    participant_handler::{PlayerHandler, TeamHandler, TeamMemberHandler},
    match_handler::{MatchHandler, MatchResultHandler},
    payment_handler::PaymentHandler,
    notification_handler::NotificationHandler,
    statistics_handler::{StatisticsHandler, AnalyticsHandler},
};

/// Configure all API routes
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    // Auth routes (no middleware)
    cfg.service(
        web::scope("/auth/google")
            .route("", web::get().to(AuthHandler::start_google_login))
            .route("/callback", web::get().to(AuthHandler::google_callback)),
    );

    // User routes
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(UserHandler::index))
            .route("", web::post().to(UserHandler::post))
            .route("/{id}", web::get().to(UserHandler::show))
            .route("/{id}", web::post().to(UserHandler::update))
            .route("/{id}", web::delete().to(UserHandler::delete)),
    );

    // User profile routes
    cfg.service(
        web::scope("/profile")
            .route("", web::get().to(UserProfileHandler::get_current_profile))
            .route("", web::put().to(UserProfileHandler::update_profile))
            .route("/{user_id}", web::get().to(UserProfileHandler::get_public_profile)),
    );

    // Player routes
    cfg.service(
        web::scope("/players")
            .route("", web::get().to(PlayerHandler::placeholder))
            .route("", web::post().to(PlayerHandler::placeholder))
            .route("/{id}", web::get().to(PlayerHandler::placeholder))
            .route("/{id}", web::post().to(PlayerHandler::placeholder))
            .route("/{id}", web::delete().to(PlayerHandler::placeholder)),
    );

    // Team routes
    cfg.service(
        web::scope("/teams")
            .route("", web::get().to(TeamHandler::placeholder))
            .route("", web::post().to(TeamHandler::placeholder))
            .route("/{id}", web::get().to(TeamHandler::placeholder))
            .route("/{id}", web::post().to(TeamHandler::placeholder))
            .route("/{id}", web::delete().to(TeamHandler::placeholder)),
    );

    // Team member routes
    cfg.service(
        web::scope("/team_members")
            .route("", web::post().to(TeamMemberHandler::placeholder)),
    );

    // Tournament routes
    cfg.service(
        web::scope("/tournaments")
            .route("", web::get().to(TournamentHandler::placeholder))
            .route("", web::post().to(TournamentHandler::placeholder))
            .route("/{id}", web::get().to(TournamentHandler::placeholder))
            .route("/{id}", web::put().to(TournamentHandler::placeholder))
            .route("/{id}", web::delete().to(TournamentHandler::placeholder)),
    );

    // Tournament category routes
    cfg.service(
        web::scope("/tournament_categories")
            .route("", web::post().to(TournamentHandler::placeholder)),
    );

    // Tournament registration routes
    cfg.service(
        web::scope("/tournament_registrations")
            .route("", web::post().to(TournamentHandler::placeholder)),
    );

    // Bracket routes
    cfg.service(
        web::scope("/brackets")
            .route("/tournament/{tournament_id}", web::get().to(TournamentHandler::placeholder)),
    );

    // Standings routes
    cfg.service(
        web::scope("/standings")
            .route("/tournament/{tournament_id}", web::get().to(TournamentHandler::placeholder)),
    );

    // Match routes
    cfg.service(
        web::scope("/matches")
            .route("", web::post().to(MatchHandler::placeholder))
            .route("/{id}", web::get().to(MatchHandler::placeholder))
            .route("/{id}", web::put().to(MatchHandler::placeholder))
            .route("/{id}", web::delete().to(MatchHandler::placeholder)),
    );

    // Match result routes
    cfg.service(
        web::scope("/match-results")
            .route("", web::post().to(MatchResultHandler::placeholder))
            .route("/{id}", web::get().to(MatchResultHandler::placeholder)),
    );

    // Notification routes
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(NotificationHandler::placeholder)),
    );

    // Payment routes
    cfg.service(
        web::scope("/payments")
            .route("/process", web::post().to(PaymentHandler::placeholder)),
    );

    // Statistics routes
    cfg.service(
        web::scope("/stats")
            .route("/player/{player_id}", web::get().to(StatisticsHandler::placeholder))
            .route("/team/{team_id}", web::get().to(StatisticsHandler::placeholder))
            .route("/tournament/{tournament_id}", web::get().to(StatisticsHandler::placeholder)),
    );

    // Analytics routes
    cfg.service(
        web::scope("/analytics")
            .route("/dashboard", web::get().to(AnalyticsHandler::placeholder)),
    );
}
