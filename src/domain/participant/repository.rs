use async_trait::async_trait;
use uuid::Uuid;

use super::entity::{Player, Team, TeamMember, TeamPlayer, TeamWithMembers};
use super::value_objects::{CreatePlayer, EditablePlayer, EditableTeam, EditableTeamMember, NewTeam, NewTeamMember};
use crate::shared::AppError;

/// Repository trait for Player entity operations
#[async_trait]
pub trait PlayerRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Player>, AppError>;
    async fn find_by_id(&self, player_id: Uuid) -> Result<Option<Player>, AppError>;
    async fn create(&self, new_player: CreatePlayer) -> Result<Player, AppError>;
    async fn update(&self, player_id: Uuid, player_data: EditablePlayer) -> Result<Option<Player>, AppError>;
    async fn delete(&self, player_id: Uuid) -> Result<Option<Player>, AppError>;
}

/// Repository trait for Team entity operations
#[async_trait]
pub trait TeamRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Team>, AppError>;
    async fn find_by_id(&self, team_id: Uuid) -> Result<Option<TeamWithMembers>, AppError>;
    async fn create(&self, new_team: NewTeam) -> Result<Team, AppError>;
    async fn update(&self, team_id: Uuid, team_data: EditableTeam) -> Result<Option<Team>, AppError>;
    async fn delete(&self, team_id: Uuid) -> Result<Option<Team>, AppError>;
}

/// Repository trait for TeamMember entity operations
#[async_trait]
pub trait TeamMemberRepository: Send + Sync {
    async fn create(&self, new_member: NewTeamMember) -> Result<TeamMember, AppError>;
    async fn get_by_team(&self, team_id: Uuid) -> Result<Vec<TeamPlayer>, AppError>;
    async fn get_by_id(&self, team_id: Uuid, player_id: Uuid) -> Result<Option<TeamMember>, AppError>;
    async fn update(&self, team_id: Uuid, player_id: Uuid, member_data: EditableTeamMember) -> Result<Option<TeamMember>, AppError>;
    async fn delete(&self, team_id: Uuid, player_id: Uuid) -> Result<Option<TeamMember>, AppError>;
}
