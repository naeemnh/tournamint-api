use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Player entity - individual participant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub user_id: Option<Uuid>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

/// Team entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
}

/// Team with its members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamWithMembers {
    #[serde(flatten)]
    pub team: Team,
    pub members: Vec<TeamPlayer>,
}

/// Team membership junction entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub team_id: Uuid,
    pub player_id: Uuid,
    pub is_captain: bool,
    pub jersey_number: Option<i32>,
    pub joined_at: DateTime<Utc>,
}

/// Player within a team context (includes team-specific info)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamPlayer {
    pub id: Uuid,
    pub name: String,
    pub user_id: Option<Uuid>,
    pub is_captain: bool,
    pub jersey_number: Option<i32>,
    pub joined_at: DateTime<Utc>,
}
