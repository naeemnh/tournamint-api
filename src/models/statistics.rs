use std::fmt::Write;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct GrowthMetrics {
    pub new_players_this_month: i64,
    pub new_teams_this_month: i64,
    pub tournaments_this_month: i64,
    pub matches_this_month: i64,
    pub revenue_this_month: Decimal,
    pub player_growth_rate: Decimal, // percentage
    pub tournament_growth_rate: Decimal, // percentage
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsFilters {
    pub sport_type: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub tournament_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardRequest {
    pub category: String, // "points", "wins", "earnings", "win_rate"
    pub entity_type: String, // "player" or "team"
    pub sport_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// Enum for different types of statistics identifiers
pub enum StatisticsIden {
    PlayersTable,
    TeamsTable,
    TournamentsTable,
    MatchesTable,
    MatchResultsTable,
    TournamentRegistrationsTable,
    // Player statistics fields
    PlayerId,
    PlayerName,
    TotalTournaments,
    TournamentsWon,
    TotalMatches,
    MatchesWon,
    MatchesLost,
    WinRate,
    TotalEarnings,
    RankingPoints,
    LastActive,
    // Team statistics fields
    TeamId,
    TeamName,
    // Tournament statistics fields
    TournamentId,
    TournamentName,
    TotalParticipants,
    CompletionRate,
    StartDate,
    EndDate,
    // Common fields
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

impl Iden for StatisticsIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                StatisticsIden::PlayersTable => "players",
                StatisticsIden::TeamsTable => "teams",
                StatisticsIden::TournamentsTable => "tournaments",
                StatisticsIden::MatchesTable => "matches",
                StatisticsIden::MatchResultsTable => "match_results",
                StatisticsIden::TournamentRegistrationsTable => "tournament_registrations",
                StatisticsIden::PlayerId => "player_id",
                StatisticsIden::PlayerName => "player_name",
                StatisticsIden::TotalTournaments => "total_tournaments",
                StatisticsIden::TournamentsWon => "tournaments_won",
                StatisticsIden::TotalMatches => "total_matches",
                StatisticsIden::MatchesWon => "matches_won",
                StatisticsIden::MatchesLost => "matches_lost",
                StatisticsIden::WinRate => "win_rate",
                StatisticsIden::TotalEarnings => "total_earnings",
                StatisticsIden::RankingPoints => "ranking_points",
                StatisticsIden::LastActive => "last_active",
                StatisticsIden::TeamId => "team_id",
                StatisticsIden::TeamName => "team_name",
                StatisticsIden::TournamentId => "tournament_id",
                StatisticsIden::TournamentName => "tournament_name",
                StatisticsIden::TotalParticipants => "total_participants",
                StatisticsIden::CompletionRate => "completion_rate",
                StatisticsIden::StartDate => "start_date",
                StatisticsIden::EndDate => "end_date",
                StatisticsIden::Id => "id",
                StatisticsIden::Name => "name",
                StatisticsIden::CreatedAt => "created_at",
                StatisticsIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}