use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct TeamMember {
    pub team_id: Uuid,
    pub player_id: Uuid,
    pub is_captain: bool,
    pub jersey_number: Option<i8>,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTeamMember {
    pub team_id: Uuid,
    pub player_id: Uuid,
    pub is_captain: Option<bool>,
    pub jersey_number: Option<i8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTeamMember {
    pub is_captain: bool,
    pub jersey_number: Option<i8>,
}

pub enum TeamMemberIden {
    Table,
    TeamId,
    PlayerId,
    IsCaptain,
    JerseyNumber,
    JoinedAt,
}

impl Iden for TeamMemberIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TeamMemberIden::Table => "team_members",
                TeamMemberIden::TeamId => "team_id",
                TeamMemberIden::PlayerId => "player_id",
                TeamMemberIden::IsCaptain => "is_captain",
                TeamMemberIden::JerseyNumber => "jersey_number",
                TeamMemberIden::JoinedAt => "joined_at",
            }
        )
        .unwrap()
    }
}
