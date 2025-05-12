use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: sqlx::types::Uuid, // Matches PostgreSQL uuid type
    pub username: String,      // VARCHAR/TEXT
    pub email: String,         // VARCHAR/TEXT
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>, // TIMESTAMPTZ
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableUser {
    pub username: String,
    pub email: String,
}
