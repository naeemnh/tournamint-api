use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::entity::TournamentBracket;

// ============ Enums (Value Objects) ============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SportType {
    Basketball,
    TableTennis,
    Volleyball,
    Badminton,
    Tennis,
    Football,
    Cricket,
    Chess,
    Esports,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TournamentFormat {
    Elimination,
    DoubleElimination,
    RoundRobin,
    League,
    Swiss,
    GroupsAndKnockout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TournamentStatus {
    Draft,
    Upcoming,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeamComposition {
    Singles,
    Doubles,
    MixedDoubles,
    Team,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrationStatus {
    Pending,
    Approved,
    Rejected,
    Withdrawn,
    Waitlisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
    Waived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BracketType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss,
    GroupStage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BracketStatus {
    NotGenerated,
    Generated,
    InProgress,
    Completed,
}

// ============ Query / Response DTOs ============

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TournamentSearchQuery {
    pub name: Option<String>,
    pub sport_type: Option<String>,
    pub status: Option<String>,
    pub format: Option<String>,
    pub location: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentStats {
    pub participants_count: i64,
    pub registrations_count: i64,
    pub categories_count: i64,
    pub matches_played: i64,
    pub prize_pool_total: String,
    pub status: TournamentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sport_type: SportType,
    pub format: TournamentFormat,
    pub default_settings: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub format: String,
    pub data: JsonValue,
    pub filename: String,
    pub content_type: String,
}

// ============ DTOs ============

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTournament {
    pub name: String,
    pub description: Option<String>,
    pub sport_type: SportType,
    pub format: TournamentFormat,
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditableTournament {
    pub name: Option<String>,
    pub description: Option<String>,
    pub sport_type: Option<SportType>,
    pub format: Option<TournamentFormat>,
    pub status: Option<TournamentStatus>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub registration_start_date: Option<DateTime<Utc>>,
    pub registration_end_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_pool: Option<Decimal>,
    pub rules: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTournamentCategory {
    pub tournament_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_composition: TeamComposition,
    pub min_participants: Option<i32>,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_distribution: Option<JsonValue>,
    pub rules: Option<JsonValue>,
    pub constraints: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTournamentCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub team_composition: Option<TeamComposition>,
    pub min_participants: Option<i32>,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_distribution: Option<JsonValue>,
    pub rules: Option<JsonValue>,
    pub constraints: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTournamentRegistration {
    pub tournament_category_id: Uuid,
    pub team_id: Option<Uuid>,
    pub player_id: Option<Uuid>,
    pub partner_player_id: Option<Uuid>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTournamentRegistration {
    pub registration_status: Option<RegistrationStatus>,
    pub payment_status: Option<PaymentStatus>,
    pub payment_amount: Option<Decimal>,
    pub payment_reference: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTournamentBracket {
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub bracket_type: BracketType,
    pub total_rounds: i32,
    pub bracket_data: Option<JsonValue>,
    pub settings: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTournamentBracket {
    pub status: Option<BracketStatus>,
    pub current_round: Option<i32>,
    pub bracket_data: Option<JsonValue>,
    pub settings: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BracketNode {
    pub id: String,
    pub round: i32,
    pub match_id: Option<Uuid>,
    pub participant1_id: Option<Uuid>,
    pub participant1_name: Option<String>,
    pub participant2_id: Option<Uuid>,
    pub participant2_name: Option<String>,
    pub winner_id: Option<Uuid>,
    pub next_match_id: Option<String>,
    pub position: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BracketGeneration {
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub bracket_type: BracketType,
    pub seed_order: Option<Vec<Uuid>>,
    pub settings: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BracketResponse {
    pub bracket: TournamentBracket,
    pub matches: Vec<BracketMatch>,
    pub participants: Vec<BracketParticipant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BracketMatch {
    pub id: Uuid,
    pub bracket_node_id: String,
    pub round: i32,
    pub position: i32,
    pub participant1_id: Option<Uuid>,
    pub participant1_name: Option<String>,
    pub participant2_id: Option<Uuid>,
    pub participant2_name: Option<String>,
    pub winner_id: Option<Uuid>,
    pub match_status: String,
    pub scheduled_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BracketParticipant {
    pub id: Uuid,
    pub name: String,
    pub seed: Option<i32>,
    pub eliminated: bool,
    pub current_round: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateBracketRequest {
    pub bracket_type: BracketType,
    pub category_id: Option<Uuid>,
    pub seed_order: Option<Vec<Uuid>>,
    pub settings: Option<JsonValue>,
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
    pub form: Vec<String>,
    pub is_eliminated: bool,
    pub elimination_round: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandingsUpdateRequest {
    pub tournament_id: Uuid,
    pub category_id: Option<Uuid>,
    pub recalculate_all: Option<bool>,
    pub match_ids: Option<Vec<Uuid>>,
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
