use std::sync::Arc;
use uuid::Uuid;

use crate::domain::participant::{
    CreatePlayer, EditablePlayer, EditableTeam, EditableTeamMember, NewTeam, NewTeamMember, Player,
    PlayerRepository, Team, TeamMember, TeamMemberRepository, TeamPlayer, TeamRepository,
    TeamWithMembers,
};
use crate::shared::AppError;

/// Participant domain services (players and teams)
pub struct ParticipantServices<P, T, M>
where
    P: PlayerRepository,
    T: TeamRepository,
    M: TeamMemberRepository,
{
    player_repo: Arc<P>,
    team_repo: Arc<T>,
    member_repo: Arc<M>,
}

impl<P, T, M> ParticipantServices<P, T, M>
where
    P: PlayerRepository,
    T: TeamRepository,
    M: TeamMemberRepository,
{
    pub fn new(player_repo: Arc<P>, team_repo: Arc<T>, member_repo: Arc<M>) -> Self {
        Self {
            player_repo,
            team_repo,
            member_repo,
        }
    }

    // ==================== Player ====================

    pub async fn get_all_players(&self) -> Result<Vec<Player>, AppError> {
        self.player_repo.find_all().await
    }

    pub async fn create_player(&self, data: CreatePlayer) -> Result<Player, AppError> {
        self.player_repo.create(data).await
    }

    pub async fn get_player(&self, player_id: Uuid) -> Result<Option<Player>, AppError> {
        self.player_repo.find_by_id(player_id).await
    }

    pub async fn update_player(
        &self,
        player_id: Uuid,
        data: EditablePlayer,
    ) -> Result<Option<Player>, AppError> {
        self.player_repo.update(player_id, data).await
    }

    pub async fn delete_player(&self, player_id: Uuid) -> Result<Option<Player>, AppError> {
        self.player_repo.delete(player_id).await
    }

    // ==================== Team ====================

    pub async fn get_all_teams(&self) -> Result<Vec<Team>, AppError> {
        self.team_repo.find_all().await
    }

    pub async fn create_team(&self, data: NewTeam) -> Result<Team, AppError> {
        self.team_repo.create(data).await
    }

    pub async fn get_team(&self, team_id: Uuid) -> Result<Option<TeamWithMembers>, AppError> {
        self.team_repo.find_by_id(team_id).await
    }

    pub async fn update_team(
        &self,
        team_id: Uuid,
        data: EditableTeam,
    ) -> Result<Option<Team>, AppError> {
        self.team_repo.update(team_id, data).await
    }

    pub async fn delete_team(&self, team_id: Uuid) -> Result<Option<Team>, AppError> {
        self.team_repo.delete(team_id).await
    }

    // ==================== Team Member ====================

    pub async fn add_team_member(&self, data: NewTeamMember) -> Result<TeamMember, AppError> {
        self.member_repo.create(data).await
    }

    pub async fn get_team_members(&self, team_id: Uuid) -> Result<Vec<TeamPlayer>, AppError> {
        self.member_repo.get_by_team(team_id).await
    }

    pub async fn get_team_members_by_player(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<TeamMember>, AppError> {
        self.member_repo.get_by_player(player_id).await
    }

    pub async fn get_team_member(
        &self,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<Option<TeamMember>, AppError> {
        self.member_repo.get_by_id(team_id, player_id).await
    }

    pub async fn update_team_member(
        &self,
        team_id: Uuid,
        player_id: Uuid,
        data: EditableTeamMember,
    ) -> Result<Option<TeamMember>, AppError> {
        self.member_repo.update(team_id, player_id, data).await
    }

    pub async fn remove_team_member(
        &self,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<Option<TeamMember>, AppError> {
        self.member_repo.delete(team_id, player_id).await
    }
}
