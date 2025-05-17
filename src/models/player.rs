use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub user_id: Option<Uuid>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>, // TIMESTAMPTZ
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditablePlayer {
    pub name: String,
    pub user_id: Option<Uuid>,
}

pub enum PlayerIden {
    Table,
    Id,
    Name,
    UserId,
    CreatedAt,
}

impl Iden for PlayerIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                PlayerIden::Table => "players",
                PlayerIden::Id => "id",
                PlayerIden::Name => "name",
                PlayerIden::UserId => "user_id",
                PlayerIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}
