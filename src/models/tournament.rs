use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::types::Json;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "sport_type", rename_all = "snake_case")]
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

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "tournament_format", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TournamentFormat {
    Elimination,
    DoubleElimination,
    RoundRobin,
    League,
    Swiss,
    GroupsAndKnockout,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "tournament_status", rename_all = "snake_case")]
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

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
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
    pub rules: Option<Json<JsonValue>>,
    pub organizer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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

#[derive(Debug, Serialize, Deserialize)]
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

pub enum TournamentIden {
    Table,
    Id,
    Name,
    Description,
    SportType,
    Format,
    Status,
    StartDate,
    EndDate,
    RegistrationStartDate,
    RegistrationEndDate,
    Venue,
    MaxParticipants,
    EntryFee,
    PrizePool,
    Rules,
    OrganizerId,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentIden::Table => "tournaments",
                TournamentIden::Id => "id",
                TournamentIden::Name => "name",
                TournamentIden::Description => "description",
                TournamentIden::SportType => "sport_type",
                TournamentIden::Format => "format",
                TournamentIden::Status => "status",
                TournamentIden::StartDate => "start_date",
                TournamentIden::EndDate => "end_date",
                TournamentIden::RegistrationStartDate => "registration_start_date",
                TournamentIden::RegistrationEndDate => "registration_end_date",
                TournamentIden::Venue => "venue",
                TournamentIden::MaxParticipants => "max_participants",
                TournamentIden::EntryFee => "entry_fee",
                TournamentIden::PrizePool => "prize_pool",
                TournamentIden::Rules => "rules",
                TournamentIden::OrganizerId => "organizer_id",
                TournamentIden::CreatedAt => "created_at",
                TournamentIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}