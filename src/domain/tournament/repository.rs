use async_trait::async_trait;
use uuid::Uuid;

use super::entity::{
    RegistrationWithDetails, Tournament, TournamentBracket, TournamentCategory,
    TournamentRegistration, TournamentStandings,
};
use super::value_objects::{
    BracketStatus, EditableTournament, EditableTournamentBracket, EditableTournamentCategory,
    EditableTournamentRegistration, EditableTournamentStandings, NewTournament,
    NewTournamentBracket, NewTournamentCategory, NewTournamentRegistration, NewTournamentStandings,
    TournamentStatus,
};
use crate::shared::AppError;

/// Repository trait for Tournament entity operations
#[async_trait]
pub trait TournamentRepository: Send + Sync {
    async fn create(&self, new_tournament: NewTournament) -> Result<Tournament, AppError>;
    async fn get_all(&self) -> Result<Vec<Tournament>, AppError>;
    async fn get_by_id(&self, tournament_id: Uuid) -> Result<Option<Tournament>, AppError>;
    async fn get_by_status(&self, status: TournamentStatus) -> Result<Vec<Tournament>, AppError>;
    async fn get_by_organizer(&self, organizer_id: Uuid) -> Result<Vec<Tournament>, AppError>;
    async fn update(&self, tournament_id: Uuid, tournament_data: EditableTournament) -> Result<Option<Tournament>, AppError>;
    async fn delete(&self, tournament_id: Uuid) -> Result<Option<Tournament>, AppError>;
}

/// Repository trait for TournamentCategory entity operations
#[async_trait]
pub trait TournamentCategoryRepository: Send + Sync {
    async fn create(&self, new_category: NewTournamentCategory) -> Result<TournamentCategory, AppError>;
    async fn get_by_tournament(&self, tournament_id: Uuid) -> Result<Vec<TournamentCategory>, AppError>;
    async fn get_by_id(&self, category_id: Uuid) -> Result<Option<TournamentCategory>, AppError>;
    async fn update(&self, category_id: Uuid, category_data: EditableTournamentCategory) -> Result<Option<TournamentCategory>, AppError>;
    async fn delete(&self, category_id: Uuid) -> Result<Option<TournamentCategory>, AppError>;
}

/// Repository trait for TournamentRegistration entity operations
#[async_trait]
pub trait TournamentRegistrationRepository: Send + Sync {
    async fn create(&self, new_registration: NewTournamentRegistration) -> Result<TournamentRegistration, AppError>;
    async fn get_by_id(&self, registration_id: Uuid) -> Result<Option<TournamentRegistration>, AppError>;
    async fn get_by_tournament_category(&self, category_id: Uuid) -> Result<Vec<TournamentRegistration>, AppError>;
    async fn get_by_tournament(&self, tournament_id: Uuid) -> Result<Vec<RegistrationWithDetails>, AppError>;
    async fn get_by_player(&self, player_id: Uuid) -> Result<Vec<RegistrationWithDetails>, AppError>;
    async fn get_by_team(&self, team_id: Uuid) -> Result<Vec<RegistrationWithDetails>, AppError>;
    async fn update(&self, registration_id: Uuid, registration_data: EditableTournamentRegistration) -> Result<Option<TournamentRegistration>, AppError>;
    async fn delete(&self, registration_id: Uuid) -> Result<Option<TournamentRegistration>, AppError>;
}

/// Repository trait for TournamentBracket entity operations
#[async_trait]
pub trait TournamentBracketRepository: Send + Sync {
    async fn create(&self, new_bracket: NewTournamentBracket) -> Result<TournamentBracket, AppError>;
    async fn get_by_tournament_id(&self, tournament_id: Uuid) -> Result<Vec<TournamentBracket>, AppError>;
    async fn get_by_category_id(&self, category_id: Uuid) -> Result<Option<TournamentBracket>, AppError>;
    async fn get_by_id(&self, bracket_id: Uuid) -> Result<Option<TournamentBracket>, AppError>;
    async fn update(&self, bracket_id: Uuid, bracket_data: EditableTournamentBracket) -> Result<Option<TournamentBracket>, AppError>;
    async fn update_status(&self, bracket_id: Uuid, status: BracketStatus) -> Result<Option<TournamentBracket>, AppError>;
    async fn delete(&self, bracket_id: Uuid) -> Result<Option<TournamentBracket>, AppError>;
    async fn exists_for_tournament(&self, tournament_id: Uuid) -> Result<bool, AppError>;
}

/// Repository trait for TournamentStandings entity operations
#[async_trait]
pub trait TournamentStandingsRepository: Send + Sync {
    async fn create(&self, new_standings: NewTournamentStandings) -> Result<TournamentStandings, AppError>;
    async fn get_by_tournament_id(&self, tournament_id: Uuid) -> Result<Vec<TournamentStandings>, AppError>;
    async fn get_by_category_id(&self, category_id: Uuid) -> Result<Vec<TournamentStandings>, AppError>;
    async fn get_by_participant(&self, participant_id: Uuid) -> Result<Vec<TournamentStandings>, AppError>;
    async fn update(&self, standings_id: Uuid, standings_data: EditableTournamentStandings) -> Result<Option<TournamentStandings>, AppError>;
    async fn delete_by_tournament(&self, tournament_id: Uuid) -> Result<u64, AppError>;
    async fn bulk_upsert(&self, standings: Vec<NewTournamentStandings>) -> Result<Vec<TournamentStandings>, AppError>;
}
