use actix_web::{web, HttpRequest, Responder};
use uuid::Uuid;

use crate::config::DbPool;
use crate::middlewares::auth::get_user_from_token;
use crate::models::common::{LeaderboardQueryParams, PaginationQuery, RecordsQuery};
use crate::models::statistics::LeaderboardRequest;
use crate::services::statistics_service::StatisticsService;

pub struct StatisticsController;

impl StatisticsController {
    /// GET /stats/player/{player_id} - Get player statistics
    pub async fn get_player_statistics(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let player_id = path.into_inner();
        StatisticsService::get_player_statistics(&pool, player_id).await
    }

    /// GET /stats/team/{team_id} - Get team statistics
    pub async fn get_team_statistics(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let team_id = path.into_inner();
        StatisticsService::get_team_statistics(&pool, team_id).await
    }

    /// GET /stats/tournament/{tournament_id} - Get tournament statistics
    pub async fn get_tournament_statistics(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let tournament_id = path.into_inner();
        StatisticsService::get_tournament_statistics(&pool, tournament_id).await
    }

    /// GET /stats/leaderboard - Get leaderboard with custom parameters
    pub async fn get_leaderboard(
        pool: web::Data<DbPool>,
        query: web::Query<LeaderboardQueryParams>,
    ) -> impl Responder {
        let request = LeaderboardRequest {
            category: query
                .category
                .clone()
                .unwrap_or_else(|| "points".to_string()),
            entity_type: query
                .entity_type
                .clone()
                .unwrap_or_else(|| "player".to_string()),
            sport_type: query.sport_type.clone(),
            limit: query.limit,
            offset: query.offset,
        };

        StatisticsService::get_leaderboard(&pool, request).await
    }

    /// GET /stats/leaderboard/players - Get player leaderboard (points-based)
    pub async fn get_player_leaderboard(
        pool: web::Data<DbPool>,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        StatisticsService::get_player_leaderboard_by_points(&pool, query.limit, query.offset).await
    }

    /// GET /stats/leaderboard/players/wins - Get player leaderboard by wins
    pub async fn get_player_leaderboard_by_wins(
        pool: web::Data<DbPool>,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        StatisticsService::get_player_leaderboard_by_wins(&pool, query.limit, query.offset).await
    }

    /// GET /stats/leaderboard/players/earnings - Get player leaderboard by earnings
    pub async fn get_player_leaderboard_by_earnings(
        pool: web::Data<DbPool>,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        StatisticsService::get_player_leaderboard_by_earnings(&pool, query.limit, query.offset)
            .await
    }

    /// GET /stats/leaderboard/teams - Get team leaderboard
    pub async fn get_team_leaderboard(
        pool: web::Data<DbPool>,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        StatisticsService::get_team_leaderboard_by_points(&pool, query.limit, query.offset).await
    }

    /// GET /stats/records - Get game records
    pub async fn get_game_records(
        pool: web::Data<DbPool>,
        query: web::Query<RecordsQuery>,
    ) -> impl Responder {
        StatisticsService::get_game_records(&pool, query.limit).await
    }

    /// GET /analytics/dashboard - Get analytics dashboard
    pub async fn get_analytics_dashboard(
        pool: web::Data<DbPool>,
        req: HttpRequest,
    ) -> impl Responder {
        // TODO: Add admin authorization check when user roles are implemented
        match get_user_from_token(&req).await {
            Ok(_user) => {
                // For now, allow any authenticated user to access dashboard
                // In production, this should check for admin/organizer role
                StatisticsService::get_analytics_dashboard(&pool).await
            }
            Err(response) => response,
        }
    }

    /// GET /analytics/growth - Get growth metrics
    pub async fn get_growth_metrics(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
        // TODO: Add admin authorization check when user roles are implemented
        match get_user_from_token(&req).await {
            Ok(_user) => StatisticsService::get_growth_metrics(&pool).await,
            Err(response) => response,
        }
    }

    /// GET /stats/player/{player_id}/comprehensive - Get comprehensive player statistics
    pub async fn get_comprehensive_player_statistics(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let player_id = path.into_inner();
        StatisticsService::get_comprehensive_player_stats(&pool, player_id).await
    }

    /// GET /stats/team/{team_id}/comprehensive - Get comprehensive team statistics
    pub async fn get_comprehensive_team_statistics(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let team_id = path.into_inner();
        StatisticsService::get_comprehensive_team_stats(&pool, team_id).await
    }

    /// GET /stats/tournament/{tournament_id}/performance - Get tournament performance summary
    pub async fn get_tournament_performance_summary(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let tournament_id = path.into_inner();
        StatisticsService::get_tournament_performance_summary(&pool, tournament_id).await
    }

    /// GET /stats/my-stats - Get current user's statistics
    pub async fn get_my_player_statistics(
        pool: web::Data<DbPool>,
        req: HttpRequest,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => {
                // Find the player record for this user
                // This assumes there's a relationship between users and players
                // You might need to adjust this based on your data model

                // For now, we'll try to get player stats using user.id as player_id
                // In a real implementation, you'd need to query the players table
                // to find the player_id associated with this user_id
                StatisticsService::get_player_statistics(&pool, user.id).await
            }
            Err(response) => response,
        }
    }

    /// GET /stats/summary - Get general platform statistics summary
    pub async fn get_platform_summary(pool: web::Data<DbPool>) -> impl Responder {
        // This returns basic platform statistics without requiring authentication
        StatisticsService::get_analytics_dashboard(&pool).await
    }
}
