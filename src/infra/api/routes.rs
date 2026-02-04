use actix_web::web;

use super::handlers::{
    auth_handler::AuthHandler,
    user_handler::{UserHandler, UserProfileHandler},
    tournament_handler::{
        TournamentBracketHandler, TournamentCategoryHandler, TournamentHandler,
        TournamentRegistrationHandler, TournamentStandingsHandler,
    },
    participant_handler::{PlayerHandler, TeamHandler, TeamMemberHandler},
    match_handler::{MatchHandler, MatchResultHandler},
    payment_handler::PaymentHandler,
    notification_handler::NotificationHandler,
    statistics_handler::{StatisticsHandler, AnalyticsHandler},
};

/// Configure all API routes
pub fn api_routes(cfg: &mut web::ServiceConfig) {
    // SSE events (real-time updates)
    cfg.service(web::scope("/events").route("", web::get().to(super::sse::event_stream)));

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
            .route("/preferences", web::post().to(UserProfileHandler::update_preferences))
            .route("/notifications", web::post().to(UserProfileHandler::update_notification_preferences))
            .route("/privacy", web::post().to(UserProfileHandler::update_privacy_settings))
            .route("/avatar", web::post().to(UserProfileHandler::update_avatar))
            .route("/avatar", web::delete().to(UserProfileHandler::remove_avatar))
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
            .route("", web::post().to(TeamMemberHandler::post))
            .route("/team/{team_id}", web::get().to(TeamMemberHandler::get_by_team))
            .route("/player/{player_id}", web::get().to(TeamMemberHandler::get_by_player))
            .route(
                "/team/{team_id}/player/{player_id}",
                web::put().to(TeamMemberHandler::update),
            )
            .route(
                "/team/{team_id}/player/{player_id}",
                web::delete().to(TeamMemberHandler::delete),
            ),
    );

    // Tournament routes (specific paths before /{id})
    cfg.service(
        web::scope("/tournaments")
            .route("", web::get().to(TournamentHandler::index))
            .route("", web::post().to(TournamentHandler::post))
            .route("/search", web::get().to(TournamentHandler::search))
            .route("/status/{status}", web::get().to(TournamentHandler::get_by_status))
            .route("/my", web::get().to(TournamentHandler::get_my))
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
            .route("", web::post().to(TournamentCategoryHandler::create_category))
            .route("/{id}", web::get().to(TournamentCategoryHandler::get_by_id))
            .route("/tournament/{tournament_id}", web::get().to(TournamentCategoryHandler::get_by_tournament))
            .route("/{id}", web::put().to(TournamentCategoryHandler::update))
            .route("/{id}", web::delete().to(TournamentCategoryHandler::delete)),
    );

    // Tournament registration routes
    cfg.service(
        web::scope("/tournament_registrations")
            .route("", web::post().to(TournamentRegistrationHandler::create_registration))
            .route("/{id}", web::get().to(TournamentRegistrationHandler::get_by_id))
            .route("/{id}", web::put().to(TournamentRegistrationHandler::update))
            .route("/{id}", web::delete().to(TournamentRegistrationHandler::delete))
            .route("/category/{category_id}", web::get().to(TournamentRegistrationHandler::get_by_category))
            .route("/tournament/{tournament_id}", web::get().to(TournamentRegistrationHandler::get_by_tournament))
            .route("/player/{player_id}", web::get().to(TournamentRegistrationHandler::get_by_player))
            .route("/team/{team_id}", web::get().to(TournamentRegistrationHandler::get_by_team)),
    );

    // Bracket routes
    cfg.service(
        web::scope("/brackets")
            .route("/tournament/{tournament_id}", web::get().to(TournamentBracketHandler::get_brackets_by_tournament))
            .route("/category/{category_id}", web::get().to(TournamentBracketHandler::get_by_category))
            .route("/generate/{tournament_id}", web::put().to(TournamentBracketHandler::generate)),
    );

    // Standings routes
    cfg.service(
        web::scope("/standings")
            .route("/tournament/{tournament_id}", web::get().to(TournamentStandingsHandler::get_standings_by_tournament))
            .route("/category/{category_id}", web::get().to(TournamentStandingsHandler::get_by_category))
            .route("/update/{tournament_id}", web::put().to(TournamentStandingsHandler::update_standings)),
    );

    // Match routes (specific paths before /{id})
    cfg.service(
        web::scope("/matches")
            .route("", web::post().to(MatchHandler::post))
            .route("/tournament/{tournament_id}", web::get().to(MatchHandler::get_by_tournament))
            .route("/category/{category_id}", web::get().to(MatchHandler::get_by_category))
            .route("/schedule", web::get().to(MatchHandler::get_schedule))
            .route("/my/upcoming", web::get().to(MatchHandler::my_upcoming))
            .route("/my/history", web::get().to(MatchHandler::my_history))
            .route("/live", web::get().to(MatchHandler::get_live))
            .route("/bulk/update", web::put().to(MatchHandler::bulk_update))
            .route("/bulk/cancel", web::put().to(MatchHandler::bulk_cancel))
            .route("/{id}", web::get().to(MatchHandler::show))
            .route("/{id}", web::put().to(MatchHandler::update))
            .route("/{id}", web::delete().to(MatchHandler::delete))
            .route("/{id}/participants", web::get().to(MatchHandler::get_participants))
            .route("/{id}/status", web::put().to(MatchHandler::update_status))
            .route("/{id}/start", web::put().to(MatchHandler::start))
            .route("/{id}/complete", web::put().to(MatchHandler::complete))
            .route("/{id}/cancel", web::put().to(MatchHandler::cancel))
            .route("/{id}/postpone", web::put().to(MatchHandler::postpone))
            .route("/{id}/reschedule", web::put().to(MatchHandler::reschedule))
            .route("/{id}/results/validate", web::get().to(MatchHandler::validate_result_scores))
            .route("/{id}/live", web::put().to(MatchHandler::update_live))
            .route("/{id}/analytics", web::get().to(MatchHandler::get_analytics))
            .route("/{id}/statistics", web::get().to(MatchHandler::get_statistics))
            .route("/{id}/media", web::get().to(MatchHandler::get_media))
            .route("/{id}/video", web::post().to(MatchHandler::upload_video))
            .route("/{id}/photo", web::post().to(MatchHandler::upload_photo))
            .route("/{id}/comments", web::get().to(MatchHandler::get_comments))
            .route("/{id}/comments", web::post().to(MatchHandler::add_comment))
            .route("/{id}/subscribe", web::post().to(MatchHandler::subscribe))
            .route("/{id}/subscribe", web::delete().to(MatchHandler::unsubscribe)),
    );

    // Match result routes
    cfg.service(
        web::scope("/match-results")
            .route("", web::post().to(MatchResultHandler::post))
            .route("/bulk", web::post().to(MatchResultHandler::bulk_post))
            .route("/{id}", web::get().to(MatchResultHandler::show))
            .route("/{id}", web::put().to(MatchResultHandler::update))
            .route("/{id}", web::delete().to(MatchResultHandler::delete))
            .route("/match/{match_id}", web::get().to(MatchResultHandler::get_by_match))
            .route("/match/{match_id}/summary", web::get().to(MatchResultHandler::get_summary))
            .route("/match/{match_id}/count", web::get().to(MatchResultHandler::get_count))
            .route("/match/{match_id}", web::delete().to(MatchResultHandler::delete_all))
            .route(
                "/match/{match_id}/set/{set_number}",
                web::get().to(MatchResultHandler::get_by_set),
            ),
    );

    // Notification routes
    cfg.service(
        web::scope("/notifications")
            .route("", web::get().to(NotificationHandler::index))
            .route("/unread", web::get().to(NotificationHandler::get_unread))
            .route("/count", web::get().to(NotificationHandler::get_count))
            .route("/read-all", web::put().to(NotificationHandler::mark_all_read))
            .route("/send", web::post().to(NotificationHandler::send))
            .route("/{id}/read", web::put().to(NotificationHandler::mark_read))
            .route("/{id}", web::delete().to(NotificationHandler::delete)),
    );

    // Payment routes
    cfg.service(
        web::scope("/payments")
            .route("/process", web::post().to(PaymentHandler::process))
            .route("/{id}", web::get().to(PaymentHandler::get))
            .route("/user/{user_id}", web::get().to(PaymentHandler::get_by_user))
            .route("/tournament/{tournament_id}", web::get().to(PaymentHandler::get_by_tournament))
            .route("/{id}/refund", web::put().to(PaymentHandler::refund))
            .route("/{id}/status", web::get().to(PaymentHandler::get_status))
            .route("/{id}/status", web::put().to(PaymentHandler::update_status))
            .route("/summary/tournament/{tournament_id}", web::get().to(PaymentHandler::get_tournament_summary))
            .route("/summary/user", web::get().to(PaymentHandler::get_user_summary)),
    );

    // Statistics routes
    cfg.service(
        web::scope("/stats")
            .route("/player/{player_id}", web::get().to(StatisticsHandler::get_player_stats))
            .route("/team/{team_id}", web::get().to(StatisticsHandler::get_team_stats))
            .route("/tournament/{tournament_id}", web::get().to(StatisticsHandler::get_tournament_stats))
            .route("/leaderboard", web::get().to(StatisticsHandler::get_leaderboard))
            .route("/leaderboard/players", web::get().to(StatisticsHandler::get_leaderboard_players))
            .route("/leaderboard/players/wins", web::get().to(StatisticsHandler::get_leaderboard_players_wins))
            .route("/leaderboard/players/earnings", web::get().to(StatisticsHandler::get_leaderboard_players_earnings))
            .route("/leaderboard/teams", web::get().to(StatisticsHandler::get_leaderboard_teams))
            .route("/records", web::get().to(StatisticsHandler::get_records))
            .route("/summary", web::get().to(StatisticsHandler::get_platform_summary))
            .route("/my-stats", web::get().to(StatisticsHandler::get_my_player_statistics)),
    );

    // Analytics routes
    cfg.service(
        web::scope("/analytics")
            .route("/dashboard", web::get().to(AnalyticsHandler::dashboard))
            .route("/growth", web::get().to(AnalyticsHandler::growth)),
    );
}
