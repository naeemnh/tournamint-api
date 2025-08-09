use actix_web::web;

use crate::controllers::statistics_controller::StatisticsController;
use crate::middlewares::auth::auth_middleware;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stats")
            // Public routes (no auth required)
            .route(
                "/player/{player_id}",
                web::get().to(StatisticsController::get_player_statistics),
            )
            .route(
                "/team/{team_id}",
                web::get().to(StatisticsController::get_team_statistics),
            )
            .route(
                "/tournament/{tournament_id}",
                web::get().to(StatisticsController::get_tournament_statistics),
            )
            .route(
                "/leaderboard",
                web::get().to(StatisticsController::get_leaderboard),
            )
            .route(
                "/leaderboard/players",
                web::get().to(StatisticsController::get_player_leaderboard),
            )
            .route(
                "/leaderboard/players/wins",
                web::get().to(StatisticsController::get_player_leaderboard_by_wins),
            )
            .route(
                "/leaderboard/players/earnings",
                web::get().to(StatisticsController::get_player_leaderboard_by_earnings),
            )
            .route(
                "/leaderboard/teams",
                web::get().to(StatisticsController::get_team_leaderboard),
            )
            .route(
                "/records",
                web::get().to(StatisticsController::get_game_records),
            )
            .route(
                "/summary",
                web::get().to(StatisticsController::get_platform_summary),
            )
            .route(
                "/player/{player_id}/comprehensive",
                web::get().to(StatisticsController::get_comprehensive_player_statistics),
            )
            .route(
                "/team/{team_id}/comprehensive",
                web::get().to(StatisticsController::get_comprehensive_team_statistics),
            )
            .route(
                "/tournament/{tournament_id}/performance",
                web::get().to(StatisticsController::get_tournament_performance_summary),
            )
            // Protected routes (require authentication)
            .service(web::scope("").wrap(auth_middleware()).route(
                "/my-stats",
                web::get().to(StatisticsController::get_my_player_statistics),
            )),
    );
}

pub fn analytics_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/analytics")
            .wrap(auth_middleware()) // All analytics routes require authentication
            .route(
                "/dashboard",
                web::get().to(StatisticsController::get_analytics_dashboard),
            )
            .route(
                "/growth",
                web::get().to(StatisticsController::get_growth_metrics),
            ),
    );
}
