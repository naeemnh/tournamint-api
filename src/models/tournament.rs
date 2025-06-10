use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar")]
pub enum TournamentCategoryType {
    SingleCategory,
    MultiCategory,
}

#[derive(Debug, Serialize, sqlx::FromRow, Deserialize)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category_type: TournamentCategoryType,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub end_date: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

pub enum TournamentIden {
    Table,
    Id,
    Name,
    Description,
    CategoryType,
    StateDate,
    EndDate,
    UpdatedAt,
    CreatedAt,
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
                TournamentIden::CategoryType => "category_type",
                TournamentIden::StateDate => "start_date",
                TournamentIden::EndDate => "end_date",
                TournamentIden::UpdatedAt => "updated_at",
                TournamentIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}
