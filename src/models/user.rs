use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub google_id: String,
    pub email: String,
    pub name: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: Option<String>,
    pub email: String,
    pub google_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableUser {
    pub name: String,
    pub email: String,
}

pub enum UserIden {
    Table,
    GoogleId,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}

impl Iden for UserIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                UserIden::Table => "users",
                UserIden::Id => "id",
                UserIden::GoogleId => "google_id",
                UserIden::Name => "name",
                UserIden::Email => "email",
                UserIden::CreatedAt => "created_at",
                UserIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}
