use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::StatisticsUseCases;
use crate::infra::db::PgStatisticsRepository;
use crate::shared::ApiResponse;

type StatisticsUseCasesData = std::sync::Arc<StatisticsUseCases<PgStatisticsRepository>>;

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

pub struct StatisticsHandler;

impl StatisticsHandler {
    pub async fn get_player_stats(
        use_cases: web::Data<StatisticsUseCasesData>,
        path: web::Path<PlayerIdPath>,
    ) -> HttpResponse {
        match use_cases
            .get_player_statistics(path.player_id, None)
            .await
        {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("Player statistics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_team_stats(
        use_cases: web::Data<StatisticsUseCasesData>,
        path: web::Path<TeamIdPath>,
    ) -> HttpResponse {
        match use_cases
            .get_team_statistics(path.team_id, None)
            .await
        {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("Team statistics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_tournament_stats(
        use_cases: web::Data<StatisticsUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases
            .get_tournament_statistics(path.tournament_id)
            .await
        {
            Ok(Some(stats)) => ApiResponse::success("OK", Some(stats)),
            Ok(None) => ApiResponse::not_found("Tournament statistics not found"),
            Err(e) => e.error_response(),
        }
    }
}

pub struct AnalyticsHandler;

impl AnalyticsHandler {
    pub async fn dashboard(
        use_cases: web::Data<StatisticsUseCasesData>,
    ) -> HttpResponse {
        match use_cases.get_analytics_dashboard().await {
            Ok(dashboard) => ApiResponse::success("OK", Some(dashboard)),
            Err(e) => e.error_response(),
        }
    }
}
