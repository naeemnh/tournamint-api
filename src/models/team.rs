use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::team_member::TeamPlayer;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct TeamWithMembers {
    #[serde(flatten)]
    pub team: Team,
    pub members: Vec<TeamPlayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTeam {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTeam {
    pub name: String,
}

pub enum TeamIden {
    Table,
    Id,
    Name,
    CreatedAt,
}

impl Iden for TeamIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TeamIden::Table => "teams",
                TeamIden::Id => "id",
                TeamIden::Name => "name",
                TeamIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}
