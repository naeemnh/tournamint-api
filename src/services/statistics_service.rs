use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::config::DbPool;
use crate::formatters;
use crate::models::statistics::{LeaderboardRequest, StatisticsFilters};
use crate::repositories::statistics_repository::StatisticsRepository;
use crate::utils::db::with_transaction;

pub struct StatisticsService;

impl StatisticsService {
    pub async fn get_player_statistics(pool: &DbPool, player_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { StatisticsRepository::get_player_statistics(tx, player_id).await })
        })
        .await
        {
            Ok(Some(stats)) => {
                formatters::success_response(StatusCode::OK, stats, "PLAYER_STATISTICS_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Player not found or no statistics available",
                "PLAYER_STATISTICS_NOT_FOUND",
            ),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_team_statistics(pool: &DbPool, team_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { StatisticsRepository::get_team_statistics(tx, team_id).await })
        })
        .await
        {
            Ok(Some(stats)) => {
                formatters::success_response(StatusCode::OK, stats, "TEAM_STATISTICS_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Team not found or no statistics available",
                "TEAM_STATISTICS_NOT_FOUND",
            ),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_tournament_statistics(pool: &DbPool, tournament_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                StatisticsRepository::get_tournament_statistics(tx, tournament_id).await
            })
        })
        .await
        {
            Ok(Some(stats)) => {
                formatters::success_response(StatusCode::OK, stats, "TOURNAMENT_STATISTICS_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Tournament not found or no statistics available",
                "TOURNAMENT_STATISTICS_NOT_FOUND",
            ),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_leaderboard(pool: &DbPool, request: LeaderboardRequest) -> HttpResponse {
        // Validate request parameters
        if !["player", "team"].contains(&request.entity_type.as_str()) {
            return formatters::error_response(
                StatusCode::BAD_REQUEST,
                "Invalid entity_type. Must be 'player' or 'team'",
                "INVALID_ENTITY_TYPE",
            );
        }

        if !["points", "wins", "earnings", "win_rate"].contains(&request.category.as_str()) {
            return formatters::error_response(
                StatusCode::BAD_REQUEST,
                "Invalid category. Must be one of: points, wins, earnings, win_rate",
                "INVALID_CATEGORY",
            );
        }

        // Limit validation
        if let Some(limit) = request.limit {
            if limit > 100 || limit < 1 {
                return formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    "Limit must be between 1 and 100",
                    "INVALID_LIMIT",
                );
            }
        }

        match with_transaction(pool, |tx| {
            Box::pin(async move { StatisticsRepository::get_leaderboard(tx, &request).await })
        })
        .await
        {
            Ok(leaderboard) => {
                formatters::success_response(StatusCode::OK, leaderboard, "LEADERBOARD_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_game_records(pool: &DbPool, limit: Option<i64>) -> HttpResponse {
        let limit = limit.unwrap_or(20).min(100);

        if limit > 100 || limit < 1 {
            return formatters::error_response(
                StatusCode::BAD_REQUEST,
                "Limit must be between 1 and 100",
                "INVALID_LIMIT",
            );
        }

        match with_transaction(pool, |tx| {
            Box::pin(async move { StatisticsRepository::get_game_records(tx, Some(limit)).await })
        })
        .await
        {
            Ok(records) => {
                formatters::success_response(StatusCode::OK, records, "GAME_RECORDS_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_analytics_dashboard(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { StatisticsRepository::get_analytics_dashboard(tx).await })
        })
        .await
        {
            Ok(dashboard) => {
                formatters::success_response(StatusCode::OK, dashboard, "ANALYTICS_DASHBOARD_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                eprintln!("Analytics dashboard error: {}", error_message);
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_growth_metrics(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { StatisticsRepository::get_growth_metrics(tx).await })
        })
        .await
        {
            Ok(metrics) => {
                formatters::success_response(StatusCode::OK, metrics, "GROWTH_METRICS_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    // Specialized leaderboard methods for easier API access
    pub async fn get_player_leaderboard_by_points(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let request = LeaderboardRequest {
            category: "points".to_string(),
            entity_type: "player".to_string(),
            sport_type: None,
            limit,
            offset,
        };
        Self::get_leaderboard(pool, request).await
    }

    pub async fn get_player_leaderboard_by_wins(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let request = LeaderboardRequest {
            category: "wins".to_string(),
            entity_type: "player".to_string(),
            sport_type: None,
            limit,
            offset,
        };
        Self::get_leaderboard(pool, request).await
    }

    pub async fn get_player_leaderboard_by_earnings(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let request = LeaderboardRequest {
            category: "earnings".to_string(),
            entity_type: "player".to_string(),
            sport_type: None,
            limit,
            offset,
        };
        Self::get_leaderboard(pool, request).await
    }

    pub async fn get_team_leaderboard_by_points(
        pool: &DbPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let request = LeaderboardRequest {
            category: "points".to_string(),
            entity_type: "team".to_string(),
            sport_type: None,
            limit,
            offset,
        };
        Self::get_leaderboard(pool, request).await
    }

    pub async fn get_comprehensive_player_stats(pool: &DbPool, player_id: Uuid) -> HttpResponse {
        // This could be extended to include additional aggregated data
        // For now, it's the same as get_player_statistics but can be enhanced
        Self::get_player_statistics(pool, player_id).await
    }

    pub async fn get_comprehensive_team_stats(pool: &DbPool, team_id: Uuid) -> HttpResponse {
        // This could be extended to include additional aggregated data
        // For now, it's the same as get_team_statistics but can be enhanced
        Self::get_team_statistics(pool, team_id).await
    }

    // Tournament-specific analytics
    pub async fn get_tournament_performance_summary(
        pool: &DbPool,
        tournament_id: Uuid,
    ) -> HttpResponse {
        // Enhanced tournament stats - for now same as basic stats but can be extended
        Self::get_tournament_statistics(pool, tournament_id).await
    }
}