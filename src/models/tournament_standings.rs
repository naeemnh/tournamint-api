use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct TournamentStandings {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub participant_id: Uuid,
    pub participant_name: String,
    pub participant_type: String, // "team", "player", "pair"
    pub position: i32,
    pub points: Decimal,
    pub matches_played: i32,
    pub matches_won: i32,
    pub matches_lost: i32,
    pub matches_drawn: i32,
    pub sets_won: i32,
    pub sets_lost: i32,
    pub games_won: i32,
    pub games_lost: i32,
    pub goal_difference: Option<i32>, // For sports with goals/points
    pub head_to_head: Option<JsonValue>,
    pub bonus_points: Option<Decimal>,
    pub penalty_points: Option<Decimal>,
    pub is_eliminated: bool,
    pub elimination_round: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTournamentStandings {
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub participant_id: Uuid,
    pub participant_name: String,
    pub participant_type: String,
    pub points: Option<Decimal>,
    pub matches_played: Option<i32>,
    pub matches_won: Option<i32>,
    pub matches_lost: Option<i32>,
    pub matches_drawn: Option<i32>,
    pub sets_won: Option<i32>,
    pub sets_lost: Option<i32>,
    pub games_won: Option<i32>,
    pub games_lost: Option<i32>,
    pub goal_difference: Option<i32>,
    pub bonus_points: Option<Decimal>,
    pub penalty_points: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTournamentStandings {
    pub position: Option<i32>,
    pub points: Option<Decimal>,
    pub matches_played: Option<i32>,
    pub matches_won: Option<i32>,
    pub matches_lost: Option<i32>,
    pub matches_drawn: Option<i32>,
    pub sets_won: Option<i32>,
    pub sets_lost: Option<i32>,
    pub games_won: Option<i32>,
    pub games_lost: Option<i32>,
    pub goal_difference: Option<i32>,
    pub head_to_head: Option<JsonValue>,
    pub bonus_points: Option<Decimal>,
    pub penalty_points: Option<Decimal>,
    pub is_eliminated: Option<bool>,
    pub elimination_round: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandingsResponse {
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub tournament_name: String,
    pub category_name: Option<String>,
    pub format: String,
    pub standings: Vec<StandingEntry>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandingEntry {
    pub position: i32,
    pub participant_id: Uuid,
    pub participant_name: String,
    pub participant_type: String,
    pub points: Decimal,
    pub matches_played: i32,
    pub matches_won: i32,
    pub matches_lost: i32,
    pub matches_drawn: i32,
    pub win_percentage: f64,
    pub sets_won: i32,
    pub sets_lost: i32,
    pub set_ratio: Option<f64>,
    pub games_won: i32,
    pub games_lost: i32,
    pub game_ratio: Option<f64>,
    pub goal_difference: Option<i32>,
    pub form: Vec<String>, // Last 5 results: "W", "L", "D"
    pub is_eliminated: bool,
    pub elimination_round: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandingsUpdateRequest {
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub recalculate_all: Option<bool>,
    pub match_ids: Option<Vec<Uuid>>, // Specific matches to update from
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipantStats {
    pub participant_id: Uuid,
    pub matches_played: i32,
    pub matches_won: i32,
    pub matches_lost: i32,
    pub matches_drawn: i32,
    pub sets_won: i32,
    pub sets_lost: i32,
    pub games_won: i32,
    pub games_lost: i32,
    pub points_scored: i32,
    pub points_conceded: i32,
}

pub enum TournamentStandingsIden {
    Table,
    Id,
    TournamentId,
    CategoryId,
    ParticipantId,
    ParticipantName,
    ParticipantType,
    Position,
    Points,
    MatchesPlayed,
    MatchesWon,
    MatchesLost,
    MatchesDrawn,
    SetsWon,
    SetsLost,
    GamesWon,
    GamesLost,
    GoalDifference,
    HeadToHead,
    BonusPoints,
    PenaltyPoints,
    IsEliminated,
    EliminationRound,
    LastUpdated,
    CreatedAt,
}

impl Iden for TournamentStandingsIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentStandingsIden::Table => "tournament_standings",
                TournamentStandingsIden::Id => "id",
                TournamentStandingsIden::TournamentId => "tournament_id",
                TournamentStandingsIden::CategoryId => "category_id",
                TournamentStandingsIden::ParticipantId => "participant_id",
                TournamentStandingsIden::ParticipantName => "participant_name",
                TournamentStandingsIden::ParticipantType => "participant_type",
                TournamentStandingsIden::Position => "position",
                TournamentStandingsIden::Points => "points",
                TournamentStandingsIden::MatchesPlayed => "matches_played",
                TournamentStandingsIden::MatchesWon => "matches_won",
                TournamentStandingsIden::MatchesLost => "matches_lost",
                TournamentStandingsIden::MatchesDrawn => "matches_drawn",
                TournamentStandingsIden::SetsWon => "sets_won",
                TournamentStandingsIden::SetsLost => "sets_lost",
                TournamentStandingsIden::GamesWon => "games_won",
                TournamentStandingsIden::GamesLost => "games_lost",
                TournamentStandingsIden::GoalDifference => "goal_difference",
                TournamentStandingsIden::HeadToHead => "head_to_head",
                TournamentStandingsIden::BonusPoints => "bonus_points",
                TournamentStandingsIden::PenaltyPoints => "penalty_points",
                TournamentStandingsIden::IsEliminated => "is_eliminated",
                TournamentStandingsIden::EliminationRound => "elimination_round",
                TournamentStandingsIden::LastUpdated => "last_updated",
                TournamentStandingsIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}