use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::StatisticsServices;
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::db::{PgPlayerRepository, PgStatisticsRepository};
use crate::shared::ApiResponse;

type StatisticsServicesData =
    std::sync::Arc<StatisticsServices<PgStatisticsRepository, PgPlayerRepository>>;

#[derive(Deserialize)]
pub struct PlayerIdPath {
    pub player_id: Uuid,
}

#[derive(Deserialize)]
pub struct TeamIdPath {
    pub team_id: Uuid,
}

#[derive(Deserialize)]
pub struct TournamentIdPath {
    pub tournament_id: Uuid,
}

#[derive(Deserialize)]
pub struct LeaderboardQuery {
    pub r#type: Option<String>,
    pub category: Option<String>,
    pub limit: Option<i64>,
}

pub struct StatisticsHandler;

impl StatisticsHandler {
    pub async fn get_player_stats(
        services: web::Data<StatisticsServicesData>,
        path: web::Path<PlayerIdPath>,
    ) -> HttpResponse {
        match services.get_player_statistics(path.player_id, None).await {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("Player statistics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_team_stats(
        services: web::Data<StatisticsServicesData>,
        path: web::Path<TeamIdPath>,
    ) -> HttpResponse {
        match services.get_team_statistics(path.team_id, None).await {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("Team statistics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_tournament_stats(
        services: web::Data<StatisticsServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match services.get_tournament_statistics(path.tournament_id).await {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("Tournament statistics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_platform_summary(services: web::Data<StatisticsServicesData>) -> HttpResponse {
        match services.get_analytics_dashboard().await {
            Ok(dashboard) => ApiResponse::success("OK", Some(dashboard)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_my_player_statistics(
        services: web::Data<StatisticsServicesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.get_my_player_statistics(user_id).await {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("No player or statistics found for current user"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_leaderboard(
        services: web::Data<StatisticsServicesData>,
        query: web::Query<LeaderboardQuery>,
    ) -> HttpResponse {
        let entity_type = query.r#type.as_deref().unwrap_or("player");
        let category = query.category.as_deref().unwrap_or("points");
        let limit = query.limit.unwrap_or(50).min(100).max(1);
        match services
            .get_leaderboard(category, entity_type, limit, 0)
            .await
        {
            Ok(entries) => ApiResponse::success("OK", Some(entries)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_leaderboard_players(
        services: web::Data<StatisticsServicesData>,
        query: web::Query<LeaderboardLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100).max(1);
        match services.get_player_leaderboard_by_points(limit).await {
            Ok(entries) => ApiResponse::success("OK", Some(entries)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_leaderboard_players_wins(
        services: web::Data<StatisticsServicesData>,
        query: web::Query<LeaderboardLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100).max(1);
        match services.get_player_leaderboard_by_wins(limit).await {
            Ok(entries) => ApiResponse::success("OK", Some(entries)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_leaderboard_players_earnings(
        services: web::Data<StatisticsServicesData>,
        query: web::Query<LeaderboardLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100).max(1);
        match services.get_player_leaderboard_by_earnings(limit).await {
            Ok(entries) => ApiResponse::success("OK", Some(entries)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_leaderboard_teams(
        services: web::Data<StatisticsServicesData>,
        query: web::Query<LeaderboardLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100).max(1);
        match services.get_team_leaderboard(limit).await {
            Ok(entries) => ApiResponse::success("OK", Some(entries)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_records(
        services: web::Data<StatisticsServicesData>,
        query: web::Query<RecordsLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100).max(1);
        match services.get_game_records(limit).await {
            Ok(records) => ApiResponse::success("OK", Some(records)),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(Deserialize)]
pub struct LeaderboardLimitQuery {
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct RecordsLimitQuery {
    pub limit: Option<i64>,
}

pub struct AnalyticsHandler;

impl AnalyticsHandler {
    pub async fn dashboard(services: web::Data<StatisticsServicesData>) -> HttpResponse {
        match services.get_analytics_dashboard().await {
            Ok(dashboard) => ApiResponse::success("OK", Some(dashboard)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn growth(services: web::Data<StatisticsServicesData>) -> HttpResponse {
        match services.get_growth_metrics().await {
            Ok(metrics) => ApiResponse::success("OK", Some(metrics)),
            Err(e) => e.error_response(),
        }
    }
}
