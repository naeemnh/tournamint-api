use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Token {
    pub refresh_token: String,
    pub user_id: uuid::Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}

pub enum TokenIden {
    Table,
    RefreshToken,
    UserId,
    ExpiresAt,
}

impl Iden for TokenIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TokenIden::Table => "user_tokens",
                TokenIden::RefreshToken => "refresh_token",
                TokenIden::UserId => "user_id",
                TokenIden::ExpiresAt => "expires_at",
            }
        )
        .unwrap()
    }
}
