use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "registration_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RegistrationStatus {
    Pending,
    Approved,
    Rejected,
    Withdrawn,
    Waitlisted,
}

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
    Waived,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
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

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
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

pub enum TournamentRegistrationIden {
    Table,
    Id,
    TournamentCategoryId,
    TeamId,
    PlayerId,
    PartnerPlayerId,
    RegistrationStatus,
    PaymentStatus,
    RegistrationDate,
    ApprovalDate,
    PaymentDate,
    PaymentAmount,
    PaymentReference,
    Notes,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentRegistrationIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentRegistrationIden::Table => "tournament_registrations",
                TournamentRegistrationIden::Id => "id",
                TournamentRegistrationIden::TournamentCategoryId => "tournament_category_id",
                TournamentRegistrationIden::TeamId => "team_id",
                TournamentRegistrationIden::PlayerId => "player_id",
                TournamentRegistrationIden::PartnerPlayerId => "partner_player_id",
                TournamentRegistrationIden::RegistrationStatus => "registration_status",
                TournamentRegistrationIden::PaymentStatus => "payment_status",
                TournamentRegistrationIden::RegistrationDate => "registration_date",
                TournamentRegistrationIden::ApprovalDate => "approval_date",
                TournamentRegistrationIden::PaymentDate => "payment_date",
                TournamentRegistrationIden::PaymentAmount => "payment_amount",
                TournamentRegistrationIden::PaymentReference => "payment_reference",
                TournamentRegistrationIden::Notes => "notes",
                TournamentRegistrationIden::Metadata => "metadata",
                TournamentRegistrationIden::CreatedAt => "created_at",
                TournamentRegistrationIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}