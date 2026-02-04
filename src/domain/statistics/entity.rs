use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Player statistics aggregation (read model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatistics {
    pub player_id: Uuid,
    pub player_name: String,
    pub total_tournaments: i64,
    pub tournaments_won: i64,
    pub tournaments_runner_up: i64,
    pub total_matches: i64,
    pub matches_won: i64,
    pub matches_lost: i64,
    pub win_rate: Decimal,
    pub total_earnings: Decimal,
    pub average_placement: Decimal,
    pub best_placement: i32,
    pub current_ranking: Option<i32>,
    pub ranking_points: Decimal,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_active: DateTime<Utc>,
}

/// Team statistics aggregation (read model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStatistics {
    pub team_id: Uuid,
    pub team_name: String,
    pub total_tournaments: i64,
    pub tournaments_won: i64,
    pub tournaments_runner_up: i64,
    pub total_matches: i64,
    pub matches_won: i64,
    pub matches_lost: i64,
    pub win_rate: Decimal,
    pub total_earnings: Decimal,
    pub average_placement: Decimal,
    pub best_placement: i32,
    pub current_ranking: Option<i32>,
    pub ranking_points: Decimal,
    pub members_count: i64,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_active: DateTime<Utc>,
}

/// Tournament statistics aggregation (read model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentStatistics {
    pub tournament_id: Uuid,
    pub tournament_name: String,
    pub total_participants: i64,
    pub total_teams: i64,
    pub total_matches: i64,
    pub completed_matches: i64,
    pub pending_matches: i64,
    pub total_prize_pool: Decimal,
    pub total_registrations: i64,
    pub completion_rate: Decimal,
    pub average_match_duration: Option<i32>, // in minutes
    pub most_wins_player: Option<String>,
    pub most_wins_team: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub end_date: Option<DateTime<Utc>>,
}

/// Leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: i64,
    pub id: Uuid,
    pub name: String,
    pub points: Decimal,
    pub tournaments_won: i64,
    pub win_rate: Decimal,
    pub total_earnings: Decimal,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub last_active: DateTime<Utc>,
}

/// Game record/achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRecord {
    pub id: Uuid,
    pub category: String, // "most_wins", "highest_earnings", "longest_streak", etc.
    pub record_type: String, // "player" or "team"
    pub holder_id: Uuid,
    pub holder_name: String,
    pub value: Decimal,
    pub description: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub achieved_date: DateTime<Utc>,
    pub tournament_id: Option<Uuid>,
    pub tournament_name: Option<String>,
}

/// Analytics dashboard data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDashboard {
    pub total_players: i64,
    pub total_teams: i64,
    pub total_tournaments: i64,
    pub active_tournaments: i64,
    pub total_matches: i64,
    pub total_earnings_distributed: Decimal,
    pub average_tournament_size: Decimal,
    pub most_popular_sport: Option<String>,
    pub top_players: Vec<LeaderboardEntry>,
    pub top_teams: Vec<LeaderboardEntry>,
    pub recent_tournaments: Vec<TournamentStatistics>,
    pub growth_metrics: GrowthMetrics,
}

/// Growth metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthMetrics {
    pub new_players_this_month: i64,
    pub new_teams_this_month: i64,
    pub tournaments_this_month: i64,
    pub matches_this_month: i64,
    pub revenue_this_month: Decimal,
    pub player_growth_rate: Decimal,
    pub tournament_growth_rate: Decimal,
}
