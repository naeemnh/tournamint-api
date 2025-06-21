use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct MatchResult {
    pub id: Uuid,
    pub match_id: Uuid,
    pub set_number: Option<i32>,
    pub participant1_score: Option<i32>,
    pub participant2_score: Option<i32>,
    pub period_number: Option<i32>,
    pub period_name: Option<String>,
    pub scoring_data: Option<JsonValue>,
    pub participant1_stats: Option<JsonValue>,
    pub participant2_stats: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMatchResult {
    pub match_id: Uuid,
    pub set_number: Option<i32>,
    pub participant1_score: Option<i32>,
    pub participant2_score: Option<i32>,
    pub period_number: Option<i32>,
    pub period_name: Option<String>,
    pub scoring_data: Option<JsonValue>,
    pub participant1_stats: Option<JsonValue>,
    pub participant2_stats: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableMatchResult {
    pub participant1_score: Option<i32>,
    pub participant2_score: Option<i32>,
    pub scoring_data: Option<JsonValue>,
    pub participant1_stats: Option<JsonValue>,
    pub participant2_stats: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchScoreSummary {
    pub match_id: Uuid,
    pub participant1_sets_won: i64,
    pub participant2_sets_won: i64,
    pub participant1_total_points: i64,
    pub participant2_total_points: i64,
}

pub enum MatchResultIden {
    Table,
    Id,
    MatchId,
    SetNumber,
    Participant1Score,
    Participant2Score,
    PeriodNumber,
    PeriodName,
    ScoringData,
    Participant1Stats,
    Participant2Stats,
    CreatedAt,
    UpdatedAt,
}

impl Iden for MatchResultIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                MatchResultIden::Table => "match_results",
                MatchResultIden::Id => "id",
                MatchResultIden::MatchId => "match_id",
                MatchResultIden::SetNumber => "set_number",
                MatchResultIden::Participant1Score => "participant1_score",
                MatchResultIden::Participant2Score => "participant2_score",
                MatchResultIden::PeriodNumber => "period_number",
                MatchResultIden::PeriodName => "period_name",
                MatchResultIden::ScoringData => "scoring_data",
                MatchResultIden::Participant1Stats => "participant1_stats",
                MatchResultIden::Participant2Stats => "participant2_stats",
                MatchResultIden::CreatedAt => "created_at",
                MatchResultIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}