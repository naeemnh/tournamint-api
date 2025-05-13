use chrono::{DateTime, Utc};
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
