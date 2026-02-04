use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Data for creating a new player
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlayer {
    pub name: String,
    pub user_id: Option<Uuid>,
}

/// Data for updating an existing player
#[derive(Debug, Serialize, Deserialize)]
pub struct EditablePlayer {
    pub name: String,
    pub user_id: Option<Uuid>,
}

/// Data for creating a new team
#[derive(Debug, Serialize, Deserialize)]
pub struct NewTeam {
    pub name: String,
}

/// Data for updating an existing team
#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTeam {
    pub name: String,
}

/// Data for adding a member to a team
#[derive(Debug, Serialize, Deserialize)]
pub struct NewTeamMember {
    pub team_id: Uuid,
    pub player_id: Uuid,
    pub is_captain: Option<bool>,
    pub jersey_number: Option<i32>,
}

/// Data for updating a team member
#[derive(Debug, Serialize, Deserialize)]
pub struct EditableTeamMember {
    pub is_captain: bool,
    pub jersey_number: Option<i32>,
}
