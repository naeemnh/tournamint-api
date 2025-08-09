use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "match_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MatchStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Postponed,
    Forfeited,
    Bye,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "match_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    GroupStage,
    RoundOf128,
    RoundOf64,
    RoundOf32,
    RoundOf16,
    QuarterFinal,
    SemiFinal,
    ThirdPlace,
    Final,
    Qualifying,
    Playoff,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Match {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub participant1_team_id: Option<Uuid>,
    pub participant1_player_id: Option<Uuid>,
    pub participant1_partner_id: Option<Uuid>,
    pub participant2_team_id: Option<Uuid>,
    pub participant2_player_id: Option<Uuid>,
    pub participant2_partner_id: Option<Uuid>,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub round_number: Option<i32>,
    pub match_number: Option<i32>,
    pub scheduled_date: DateTime<Utc>,
    pub actual_start_date: Option<DateTime<Utc>>,
    pub actual_end_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub winner_participant: Option<i32>,
    pub is_draw: bool,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMatch {
    pub tournament_category_id: Uuid,
    pub participant1_team_id: Option<Uuid>,
    pub participant1_player_id: Option<Uuid>,
    pub participant1_partner_id: Option<Uuid>,
    pub participant2_team_id: Option<Uuid>,
    pub participant2_player_id: Option<Uuid>,
    pub participant2_partner_id: Option<Uuid>,
    pub match_type: MatchType,
    pub round_number: Option<i32>,
    pub match_number: Option<i32>,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableMatch {
    pub match_status: Option<MatchStatus>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct MatchWithParticipants {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub participant1_name: String,
    pub participant2_name: String,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub winner_participant: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchStatusUpdate {
    pub match_status: MatchStatus,
    pub winner_participant: Option<i32>,
    pub is_draw: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchScheduleRequest {
    pub tournament_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub venue: Option<String>,
    pub status: Option<MatchStatus>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct MatchScheduleItem {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub tournament_name: String,
    pub category_name: String,
    pub participant1_name: String,
    pub participant2_name: String,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub round_number: Option<i32>,
}

// Request DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMatchRequest {
    pub tournament_category_id: Uuid,
    pub participant1_team_id: Option<Uuid>,
    pub participant1_player_id: Option<Uuid>,
    pub participant1_partner_id: Option<Uuid>,
    pub participant2_team_id: Option<Uuid>,
    pub participant2_player_id: Option<Uuid>,
    pub participant2_partner_id: Option<Uuid>,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub round_number: Option<i32>,
    pub match_number: Option<i32>,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMatchRequest {
    pub scheduled_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMatchStatusRequest {
    pub status: MatchStatus,
    pub winner_participant: Option<i32>,
    pub is_draw: Option<bool>,
}

pub enum MatchIden {
    Table,
    Id,
    TournamentCategoryId,
    Participant1TeamId,
    Participant1PlayerId,
    Participant1PartnerId,
    Participant2TeamId,
    Participant2PlayerId,
    Participant2PartnerId,
    MatchType,
    MatchStatus,
    RoundNumber,
    MatchNumber,
    ScheduledDate,
    ActualStartDate,
    ActualEndDate,
    Venue,
    CourtNumber,
    WinnerParticipant,
    IsDraw,
    RefereeName,
    UmpireName,
    Notes,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

impl Iden for MatchIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                MatchIden::Table => "matches",
                MatchIden::Id => "id",
                MatchIden::TournamentCategoryId => "tournament_category_id",
                MatchIden::Participant1TeamId => "participant1_team_id",
                MatchIden::Participant1PlayerId => "participant1_player_id",
                MatchIden::Participant1PartnerId => "participant1_partner_id",
                MatchIden::Participant2TeamId => "participant2_team_id",
                MatchIden::Participant2PlayerId => "participant2_player_id",
                MatchIden::Participant2PartnerId => "participant2_partner_id",
                MatchIden::MatchType => "match_type",
                MatchIden::MatchStatus => "match_status",
                MatchIden::RoundNumber => "round_number",
                MatchIden::MatchNumber => "match_number",
                MatchIden::ScheduledDate => "scheduled_date",
                MatchIden::ActualStartDate => "actual_start_date",
                MatchIden::ActualEndDate => "actual_end_date",
                MatchIden::Venue => "venue",
                MatchIden::CourtNumber => "court_number",
                MatchIden::WinnerParticipant => "winner_participant",
                MatchIden::IsDraw => "is_draw",
                MatchIden::RefereeName => "referee_name",
                MatchIden::UmpireName => "umpire_name",
                MatchIden::Notes => "notes",
                MatchIden::Metadata => "metadata",
                MatchIden::CreatedAt => "created_at",
                MatchIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}