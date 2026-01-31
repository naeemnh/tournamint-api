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
            .route("", web::get().to(PlayerHandler::index))
            .route("", web::post().to(PlayerHandler::post))
            .route("/{id}", web::get().to(PlayerHandler::show))
            .route("/{id}", web::post().to(PlayerHandler::update))
            .route("/{id}", web::delete().to(PlayerHandler::delete)),
    );

    // Team routes
    cfg.service(
        web::scope("/teams")
            .route("", web::get().to(TeamHandler::index))
            .route("", web::post().to(TeamHandler::post))
            .route("/{id}", web::get().to(TeamHandler::show))
            .route("/{id}", web::post().to(TeamHandler::update))
            .route("/{id}", web::delete().to(TeamHandler::delete)),
    );

    // Team member routes
    cfg.service(
        web::scope("/team_members")
            .route("", web::post().to(TeamMemberHandler::post)),
    );

    // Tournament routes (specific paths before /{id})
    cfg.service(
        web::scope("/tournaments")
            .route("", web::get().to(TournamentHandler::index))
            .route("", web::post().to(TournamentHandler::post))
            .route("/search", web::get().to(TournamentHandler::search))
            .route("/featured", web::get().to(TournamentHandler::get_featured))
            .route("/upcoming", web::get().to(TournamentHandler::get_upcoming))
            .route("/templates", web::get().to(TournamentHandler::get_templates))
            .route("/templates/{template_id}", web::post().to(TournamentHandler::create_from_template))
            .route("/{id}", web::get().to(TournamentHandler::show))
            .route("/{id}", web::put().to(TournamentHandler::update))
            .route("/{id}", web::delete().to(TournamentHandler::delete))
            .route("/{id}/publish", web::put().to(TournamentHandler::publish))
            .route("/{id}/start", web::put().to(TournamentHandler::start))
            .route("/{id}/complete", web::put().to(TournamentHandler::complete))
            .route("/{id}/cancel", web::put().to(TournamentHandler::cancel))
            .route("/{id}/stats", web::get().to(TournamentHandler::get_stats))
            .route("/{id}/participants", web::get().to(TournamentHandler::get_participants))
            .route("/{id}/export", web::get().to(TournamentHandler::export))
            .route("/{id}/duplicate", web::post().to(TournamentHandler::duplicate))
            .route("/{id}/dashboard", web::get().to(TournamentHandler::get_dashboard))
            .route("/{id}/settings", web::put().to(TournamentHandler::update_settings)),
    );

    // Tournament category routes
    cfg.service(
        web::scope("/tournament_categories")
            .route("", web::post().to(TournamentHandler::create_category)),
    );

    // Tournament registration routes
    cfg.service(
        web::scope("/tournament_registrations")
            .route("", web::post().to(TournamentHandler::create_registration)),
    );

    // Bracket routes
    cfg.service(
        web::scope("/brackets")
            .route("/tournament/{tournament_id}", web::get().to(TournamentHandler::get_brackets_by_tournament)),
    );

    // Standings routes
    cfg.service(
        web::scope("/standings")
            .route("/tournament/{tournament_id}", web::get().to(TournamentHandler::get_standings_by_tournament)),
    );

    // Match routes
    cfg.service(
        web::scope("/matches")
            .route("", web::post().to(MatchHandler::post))
            .route("/{id}", web::get().to(MatchHandler::show))
            .route("/{id}", web::put().to(MatchHandler::update))
            .route("/{id}", web::delete().to(MatchHandler::delete))
            .route("/{id}/reschedule", web::put().to(MatchHandler::reschedule))
            .route("/{id}/results/validate", web::get().to(MatchHandler::validate_result_scores)),
    );

    // Match result routes
    cfg.service(
        web::scope("/match-results")
            .route("", web::post().to(MatchResultHandler::post))
            .route("/bulk", web::post().to(MatchResultHandler::bulk_post))
            .route("/{id}", web::get().to(MatchResultHandler::show)),
    );

    // Notification routes
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(NotificationHandler::index)),
    );

    // Payment routes
    cfg.service(
        web::scope("/payments")
            .route("/process", web::post().to(PaymentHandler::process)),
    );

    // Statistics routes
    cfg.service(
        web::scope("/stats")
            .route("/player/{player_id}", web::get().to(StatisticsHandler::get_player_stats))
            .route("/team/{team_id}", web::get().to(StatisticsHandler::get_team_stats))
            .route("/tournament/{tournament_id}", web::get().to(StatisticsHandler::get_tournament_stats))
            .route("/summary", web::get().to(StatisticsHandler::get_platform_summary))
            .route("/my-stats", web::get().to(StatisticsHandler::get_my_player_statistics)),
    );

    // Analytics routes
    cfg.service(
        web::scope("/analytics")
            .route("/dashboard", web::get().to(AnalyticsHandler::dashboard)),
    );
}
