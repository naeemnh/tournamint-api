use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::value_objects::{
    BracketStatus, BracketType, PaymentStatus, RegistrationStatus, SportType, TeamComposition,
    TournamentFormat, TournamentStatus,
};

/// Core tournament entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub sport_type: SportType,
    pub format: TournamentFormat,
    pub status: TournamentStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub registration_start_date: Option<DateTime<Utc>>,
    pub registration_end_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_pool: Option<Decimal>,
    pub rules: Option<JsonValue>,
    pub organizer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tournament category (e.g., Singles, Doubles, Age groups)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentCategory {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_composition: TeamComposition,
    pub min_participants: i32,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_distribution: Option<JsonValue>,
    pub rules: Option<JsonValue>,
    pub constraints: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Registration for a tournament category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentRegistration {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub team_id: Option<Uuid>,
    pub player_id: Option<Uuid>,
    pub partner_player_id: Option<Uuid>,
    pub registration_status: RegistrationStatus,
    pub payment_status: PaymentStatus,
    pub registration_date: DateTime<Utc>,
    pub approval_date: Option<DateTime<Utc>>,
    pub payment_date: Option<DateTime<Utc>>,
    pub payment_amount: Option<Decimal>,
    pub payment_reference: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Registration with joined details for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationWithDetails {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub tournament_name: String,
    pub category_name: String,
    pub team_name: Option<String>,
    pub player_name: Option<String>,
    pub partner_name: Option<String>,
    pub registration_status: RegistrationStatus,
    pub payment_status: PaymentStatus,
    pub registration_date: DateTime<Utc>,
}

/// Tournament bracket structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentBracket {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub bracket_type: BracketType,
    pub status: BracketStatus,
    pub total_rounds: i32,
    pub current_round: i32,
    pub bracket_data: Option<JsonValue>,
    pub settings: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tournament standings/leaderboard entry
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub goal_difference: Option<i32>,
    pub head_to_head: Option<JsonValue>,
    pub bonus_points: Option<Decimal>,
    pub penalty_points: Option<Decimal>,
    pub is_eliminated: bool,
    pub elimination_round: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
