use std::sync::Arc;
use uuid::Uuid;

use crate::domain::tournament::{
    EditableTournament, EditableTournamentBracket, EditableTournamentCategory,
    EditableTournamentRegistration, EditableTournamentStandings, NewTournament,
    NewTournamentBracket, NewTournamentCategory, NewTournamentRegistration,
    NewTournamentStandings, RegistrationWithDetails, Tournament, TournamentBracket,
    TournamentBracketRepository, TournamentCategory, TournamentCategoryRepository,
    TournamentRegistration, TournamentRegistrationRepository, TournamentRepository,
    TournamentStandings, TournamentStandingsRepository, TournamentStatus,
};
use crate::shared::AppError;

/// Tournament domain use cases
pub struct TournamentUseCases<T, C, R, B, S>
where
    T: TournamentRepository,
    C: TournamentCategoryRepository,
    R: TournamentRegistrationRepository,
    B: TournamentBracketRepository,
    S: TournamentStandingsRepository,
{
    tournament_repo: Arc<T>,
    category_repo: Arc<C>,
    registration_repo: Arc<R>,
    bracket_repo: Arc<B>,
    standings_repo: Arc<S>,
}

impl<T, C, R, B, S> TournamentUseCases<T, C, R, B, S>
where
    T: TournamentRepository,
    C: TournamentCategoryRepository,
    R: TournamentRegistrationRepository,
    B: TournamentBracketRepository,
    S: TournamentStandingsRepository,
{
    pub fn new(
        tournament_repo: Arc<T>,
        category_repo: Arc<C>,
        registration_repo: Arc<R>,
        bracket_repo: Arc<B>,
        standings_repo: Arc<S>,
    ) -> Self {
        Self {
            tournament_repo,
            category_repo,
            registration_repo,
            bracket_repo,
            standings_repo,
        }
    }

    // ==================== Tournament CRUD ====================

    pub async fn create_tournament(&self, data: NewTournament) -> Result<Tournament, AppError> {
        self.tournament_repo.create(data).await
    }

    pub async fn get_all_tournaments(&self) -> Result<Vec<Tournament>, AppError> {
        self.tournament_repo.get_all().await
    }

    pub async fn get_tournament_by_id(&self, id: Uuid) -> Result<Option<Tournament>, AppError> {
        self.tournament_repo.get_by_id(id).await
    }

    pub async fn get_tournaments_by_status(
        &self,
        status: TournamentStatus,
    ) -> Result<Vec<Tournament>, AppError> {
        self.tournament_repo.get_by_status(status).await
    }

    pub async fn get_my_tournaments(&self, organizer_id: Uuid) -> Result<Vec<Tournament>, AppError> {
        self.tournament_repo.get_by_organizer(organizer_id).await
    }

    pub async fn update_tournament(
        &self,
        id: Uuid,
        data: EditableTournament,
    ) -> Result<Option<Tournament>, AppError> {
        self.tournament_repo.update(id, data).await
    }

    pub async fn delete_tournament(&self, id: Uuid) -> Result<Option<Tournament>, AppError> {
        self.tournament_repo.delete(id).await
    }

    // ==================== Category ====================

    pub async fn create_category(
        &self,
        data: NewTournamentCategory,
    ) -> Result<TournamentCategory, AppError> {
        self.category_repo.create(data).await
    }

    pub async fn get_categories_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentCategory>, AppError> {
        self.category_repo.get_by_tournament(tournament_id).await
    }

    pub async fn get_category_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TournamentCategory>, AppError> {
        self.category_repo.get_by_id(id).await
    }

    pub async fn update_category(
        &self,
        id: Uuid,
        data: EditableTournamentCategory,
    ) -> Result<Option<TournamentCategory>, AppError> {
        self.category_repo.update(id, data).await
    }

    pub async fn delete_category(&self, id: Uuid) -> Result<Option<TournamentCategory>, AppError> {
        self.category_repo.delete(id).await
    }

    // ==================== Registration ====================

    pub async fn create_registration(
        &self,
        data: NewTournamentRegistration,
    ) -> Result<TournamentRegistration, AppError> {
        self.registration_repo.create(data).await
    }

    pub async fn get_registration_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TournamentRegistration>, AppError> {
        self.registration_repo.get_by_id(id).await
    }

    pub async fn get_registrations_by_category(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>, AppError> {
        self.registration_repo
            .get_by_tournament_category(category_id)
            .await
    }

    pub async fn get_registrations_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, AppError> {
        self.registration_repo.get_by_tournament(tournament_id).await
    }

    pub async fn get_registrations_by_player(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, AppError> {
        self.registration_repo.get_by_player(player_id).await
    }

    pub async fn get_registrations_by_team(
        &self,
        team_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, AppError> {
        self.registration_repo.get_by_team(team_id).await
    }

    pub async fn update_registration(
        &self,
        id: Uuid,
        data: EditableTournamentRegistration,
    ) -> Result<Option<TournamentRegistration>, AppError> {
        self.registration_repo.update(id, data).await
    }

    pub async fn delete_registration(
        &self,
        id: Uuid,
    ) -> Result<Option<TournamentRegistration>, AppError> {
        self.registration_repo.delete(id).await
    }

    // ==================== Bracket ====================

    pub async fn create_bracket(
        &self,
        data: NewTournamentBracket,
    ) -> Result<TournamentBracket, AppError> {
        self.bracket_repo.create(data).await
    }

    pub async fn get_brackets_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentBracket>, AppError> {
        self.bracket_repo.get_by_tournament_id(tournament_id).await
    }

    pub async fn get_bracket_by_category(
        &self,
        category_id: Uuid,
    ) -> Result<Option<TournamentBracket>, AppError> {
        self.bracket_repo.get_by_category_id(category_id).await
    }

    pub async fn update_bracket(
        &self,
        id: Uuid,
        data: EditableTournamentBracket,
    ) -> Result<Option<TournamentBracket>, AppError> {
        self.bracket_repo.update(id, data).await
    }

    pub async fn delete_bracket(&self, id: Uuid) -> Result<Option<TournamentBracket>, AppError> {
        self.bracket_repo.delete(id).await
    }

    // ==================== Standings ====================

    pub async fn create_standings(
        &self,
        data: NewTournamentStandings,
    ) -> Result<TournamentStandings, AppError> {
        self.standings_repo.create(data).await
    }

    pub async fn get_standings_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, AppError> {
        self.standings_repo.get_by_tournament_id(tournament_id).await
    }

    pub async fn get_standings_by_category(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, AppError> {
        self.standings_repo.get_by_category_id(category_id).await
    }

    pub async fn update_standings(
        &self,
        id: Uuid,
        data: EditableTournamentStandings,
    ) -> Result<Option<TournamentStandings>, AppError> {
        self.standings_repo.update(id, data).await
    }

    pub async fn recalculate_standings(
        &self,
        tournament_id: Uuid,
    ) -> Result<u64, AppError> {
        // First delete existing standings, then they would need to be recalculated
        // This is a placeholder - actual implementation would involve complex calculations
        self.standings_repo.delete_by_tournament(tournament_id).await
    }
}
