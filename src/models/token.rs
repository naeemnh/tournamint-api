use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserToken {
    pub refresh_token: String,
    pub user_id: uuid::Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}

pub enum UserTokenIden {
    Table,
    RefreshToken,
    UserId,
    ExpiresAt,
}

impl Iden for UserTokenIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                UserTokenIden::Table => "user_tokens",
                UserTokenIden::RefreshToken => "refresh_token",
                UserTokenIden::UserId => "user_id",
                UserTokenIden::ExpiresAt => "expires_at",
            }
        )
        .unwrap()
    }
}
