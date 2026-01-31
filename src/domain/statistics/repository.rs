use async_trait::async_trait;
use uuid::Uuid;

use super::entity::{
    AnalyticsDashboard, GameRecord, GrowthMetrics, LeaderboardEntry, PlayerStatistics,
    TeamStatistics, TournamentStatistics,
};
use super::value_objects::StatisticsFilters;
use crate::shared::AppError;

/// Repository trait for Statistics (read-only operations)
#[async_trait]
pub trait StatisticsRepository: Send + Sync {
    async fn get_player_statistics(&self, player_id: Uuid, filters: Option<StatisticsFilters>) -> Result<Option<PlayerStatistics>, AppError>;
    async fn get_team_statistics(&self, team_id: Uuid, filters: Option<StatisticsFilters>) -> Result<Option<TeamStatistics>, AppError>;
    async fn get_tournament_statistics(&self, tournament_id: Uuid) -> Result<Option<TournamentStatistics>, AppError>;
    async fn get_leaderboard(&self, category: &str, entity_type: &str, limit: i64, offset: i64) -> Result<Vec<LeaderboardEntry>, AppError>;
    async fn get_game_records(&self, limit: i64) -> Result<Vec<GameRecord>, AppError>;
    async fn get_growth_metrics(&self) -> Result<GrowthMetrics, AppError>;
    async fn get_analytics_dashboard(&self) -> Result<AnalyticsDashboard, AppError>;
}
