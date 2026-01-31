use std::sync::Arc;
use uuid::Uuid;

use crate::domain::statistics::{
    AnalyticsDashboard, GameRecord, GrowthMetrics, LeaderboardEntry, PlayerStatistics,
    StatisticsFilters, StatisticsRepository, TeamStatistics, TournamentStatistics,
};
use crate::shared::AppError;

/// Statistics domain use cases (read-only analytics)
pub struct StatisticsUseCases<R>
where
    R: StatisticsRepository,
{
    stats_repo: Arc<R>,
}

impl<R> StatisticsUseCases<R>
where
    R: StatisticsRepository,
{
    pub fn new(stats_repo: Arc<R>) -> Self {
        Self { stats_repo }
    }

    pub async fn get_player_statistics(
        &self,
        player_id: Uuid,
        filters: Option<StatisticsFilters>,
    ) -> Result<Option<PlayerStatistics>, AppError> {
        self.stats_repo.get_player_statistics(player_id, filters).await
    }

    pub async fn get_team_statistics(
        &self,
        team_id: Uuid,
        filters: Option<StatisticsFilters>,
    ) -> Result<Option<TeamStatistics>, AppError> {
        self.stats_repo.get_team_statistics(team_id, filters).await
    }

    pub async fn get_tournament_statistics(
        &self,
        tournament_id: Uuid,
    ) -> Result<Option<TournamentStatistics>, AppError> {
        self.stats_repo.get_tournament_statistics(tournament_id).await
    }

    pub async fn get_leaderboard(
        &self,
        category: &str,
        entity_type: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        self.stats_repo
            .get_leaderboard(category, entity_type, limit, offset)
            .await
    }

    pub async fn get_player_leaderboard_by_points(
        &self,
        limit: i64,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        self.stats_repo
            .get_leaderboard("points", "player", limit, 0)
            .await
    }

    pub async fn get_player_leaderboard_by_wins(
        &self,
        limit: i64,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        self.stats_repo
            .get_leaderboard("wins", "player", limit, 0)
            .await
    }

    pub async fn get_player_leaderboard_by_earnings(
        &self,
        limit: i64,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        self.stats_repo
            .get_leaderboard("earnings", "player", limit, 0)
            .await
    }

    pub async fn get_team_leaderboard(
        &self,
        limit: i64,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        self.stats_repo
            .get_leaderboard("points", "team", limit, 0)
            .await
    }

    pub async fn get_game_records(&self, limit: i64) -> Result<Vec<GameRecord>, AppError> {
        self.stats_repo.get_game_records(limit).await
    }

    pub async fn get_growth_metrics(&self) -> Result<GrowthMetrics, AppError> {
        self.stats_repo.get_growth_metrics().await
    }

    pub async fn get_analytics_dashboard(&self) -> Result<AnalyticsDashboard, AppError> {
        self.stats_repo.get_analytics_dashboard().await
    }
}
