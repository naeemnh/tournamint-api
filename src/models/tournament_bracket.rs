use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "bracket_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BracketType {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss,
    GroupStage,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "bracket_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum BracketStatus {
    NotGenerated,
    Generated,
    InProgress,
    Completed,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
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
    pub seed_order: Option<Vec<Uuid>>, // Ordered participant IDs
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

// Request DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateBracketRequest {
    pub bracket_type: BracketType,
    pub category_id: Option<Uuid>,
    pub seed_order: Option<Vec<Uuid>>,
    pub settings: Option<JsonValue>,
}

pub enum TournamentBracketIden {
    Table,
    Id,
    TournamentId,
    CategoryId,
    BracketType,
    Status,
    TotalRounds,
    CurrentRound,
    BracketData,
    Settings,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentBracketIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentBracketIden::Table => "tournament_brackets",
                TournamentBracketIden::Id => "id",
                TournamentBracketIden::TournamentId => "tournament_id",
                TournamentBracketIden::CategoryId => "category_id",
                TournamentBracketIden::BracketType => "bracket_type",
                TournamentBracketIden::Status => "status",
                TournamentBracketIden::TotalRounds => "total_rounds",
                TournamentBracketIden::CurrentRound => "current_round",
                TournamentBracketIden::BracketData => "bracket_data",
                TournamentBracketIden::Settings => "settings",
                TournamentBracketIden::CreatedAt => "created_at",
                TournamentBracketIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}