use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Player {
    id: Uuid,
    name: String,
    user_id: Option<String>,
    created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPlayer {
    name: String,
    user_id: Option<Uuid>,
}
