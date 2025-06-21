use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::types::Json;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "team_composition", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TeamComposition {
    Singles,
    Doubles,
    MixedDoubles,
    Team,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct TournamentCategory {
    pub id: Uuid,
    pub tournament_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub team_composition: TeamComposition,
    pub min_participants: i32,
    pub max_participants: Option<i32>,
    pub entry_fee: Option<Decimal>,
    pub prize_distribution: Option<Json<JsonValue>>,
    pub rules: Option<Json<JsonValue>>,
    pub constraints: Option<Json<JsonValue>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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

pub enum TournamentCategoryIden {
    Table,
    Id,
    TournamentId,
    Name,
    Description,
    TeamComposition,
    MinParticipants,
    MaxParticipants,
    EntryFee,
    PrizeDistribution,
    Rules,
    Constraints,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentCategoryIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentCategoryIden::Table => "tournament_categories",
                TournamentCategoryIden::Id => "id",
                TournamentCategoryIden::TournamentId => "tournament_id",
                TournamentCategoryIden::Name => "name",
                TournamentCategoryIden::Description => "description",
                TournamentCategoryIden::TeamComposition => "team_composition",
                TournamentCategoryIden::MinParticipants => "min_participants",
                TournamentCategoryIden::MaxParticipants => "max_participants",
                TournamentCategoryIden::EntryFee => "entry_fee",
                TournamentCategoryIden::PrizeDistribution => "prize_distribution",
                TournamentCategoryIden::Rules => "rules",
                TournamentCategoryIden::Constraints => "constraints",
                TournamentCategoryIden::CreatedAt => "created_at",
                TournamentCategoryIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}