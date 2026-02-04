use std::sync::Arc;
use uuid::Uuid;

use chrono::Duration;
use serde_json::Value as JsonValue;

use crate::domain::tournament::{
    BracketType, EditableTournament, EditableTournamentBracket, EditableTournamentCategory,
    EditableTournamentRegistration, EditableTournamentStandings, ExportData, NewTournament,
    NewTournamentBracket, NewTournamentCategory, NewTournamentRegistration,
    NewTournamentStandings, RegistrationWithDetails, SportType, Tournament, TournamentBracket,
    TournamentBracketRepository, TournamentCategory, TournamentCategoryRepository,
    TournamentDashboard, TournamentFormat, TournamentRegistration,
    TournamentRegistrationRepository, TournamentRepository, TournamentStandings,
    TournamentStandingsRepository, TournamentStatus, TournamentSearchQuery, TournamentStats,
    TournamentTemplate,
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

    pub async fn search_tournaments(
        &self,
        query: TournamentSearchQuery,
    ) -> Result<Vec<Tournament>, AppError> {
        self.tournament_repo.search(query).await
    }

    pub async fn get_featured_tournaments(
        &self,
        limit: u32,
    ) -> Result<Vec<Tournament>, AppError> {
        self.tournament_repo.get_featured(limit).await
    }

    pub async fn get_upcoming_tournaments(&self) -> Result<Vec<Tournament>, AppError> {
        self.tournament_repo.get_upcoming().await
    }

    pub async fn publish_tournament(&self, id: Uuid) -> Result<Option<Tournament>, AppError> {
        let data = EditableTournament {
            status: Some(TournamentStatus::RegistrationOpen),
            ..Default::default()
        };
        self.tournament_repo.update(id, data).await
    }

    pub async fn start_tournament(&self, id: Uuid) -> Result<Option<Tournament>, AppError> {
        let data = EditableTournament {
            status: Some(TournamentStatus::InProgress),
            ..Default::default()
        };
        self.tournament_repo.update(id, data).await
    }

    pub async fn complete_tournament(&self, id: Uuid) -> Result<Option<Tournament>, AppError> {
        let data = EditableTournament {
            status: Some(TournamentStatus::Completed),
            ..Default::default()
        };
        self.tournament_repo.update(id, data).await
    }

    pub async fn cancel_tournament(
        &self,
        id: Uuid,
        reason: Option<String>,
    ) -> Result<Option<Tournament>, AppError> {
        let mut rules = serde_json::Map::new();
        if let Some(r) = reason {
            rules.insert(
                "cancellation_reason".to_string(),
                serde_json::Value::String(r),
            );
        }
        let data = EditableTournament {
            status: Some(TournamentStatus::Cancelled),
            rules: Some(JsonValue::Object(rules)),
            ..Default::default()
        };
        self.tournament_repo.update(id, data).await
    }

    pub async fn get_tournament_stats(&self, id: Uuid) -> Result<TournamentStats, AppError> {
        self.tournament_repo.get_tournament_stats(id).await
    }

    pub async fn get_tournament_participants(
        &self,
        id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, AppError> {
        self.registration_repo.get_by_tournament(id).await
    }

    pub async fn export_tournament(
        &self,
        id: Uuid,
        format: String,
    ) -> Result<ExportData, AppError> {
        let tournament = self
            .tournament_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tournament not found".into()))?;
        let categories = self.category_repo.get_by_tournament(id).await?;
        let registrations = self.registration_repo.get_by_tournament(id).await?;

        let mut export_data = serde_json::Map::new();
        export_data.insert(
            "tournament".to_string(),
            serde_json::to_value(&tournament).map_err(|e| AppError::InternalError(e.to_string()))?,
        );
        export_data.insert(
            "categories".to_string(),
            serde_json::to_value(&categories).map_err(|e| AppError::InternalError(e.to_string()))?,
        );
        export_data.insert(
            "registrations".to_string(),
            serde_json::to_value(&registrations).map_err(|e| AppError::InternalError(e.to_string()))?,
        );
        export_data.insert(
            "exported_at".to_string(),
            serde_json::to_value(chrono::Utc::now()).map_err(|e| AppError::InternalError(e.to_string()))?,
        );

        let (content_type, filename) = match format.as_str() {
            "csv" => ("text/csv".to_string(), format!("{}_export.csv", tournament.name)),
            "pdf" => ("application/pdf".to_string(), format!("{}_export.pdf", tournament.name)),
            _ => (
                "application/json".to_string(),
                format!("{}_export.json", tournament.name),
            ),
        };

        Ok(ExportData {
            format: format.clone(),
            data: JsonValue::Object(export_data),
            filename,
            content_type,
        })
    }

    pub async fn duplicate_tournament(
        &self,
        id: Uuid,
        new_name: String,
    ) -> Result<Tournament, AppError> {
        let original = self
            .tournament_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tournament not found".into()))?;

        let week = Duration::days(7);
        let new_tournament = NewTournament {
            name: new_name,
            description: original.description.clone(),
            sport_type: original.sport_type,
            format: original.format,
            start_date: original.start_date + week,
            end_date: original.end_date + week,
            registration_start_date: original
                .registration_start_date
                .map(|d| d + week),
            registration_end_date: original.registration_end_date.map(|d| d + week),
            venue: original.venue.clone(),
            max_participants: original.max_participants,
            entry_fee: original.entry_fee,
            prize_pool: original.prize_pool,
            rules: original.rules.clone(),
            organizer_id: original.organizer_id,
        };
        self.tournament_repo.create(new_tournament).await
    }

    pub async fn get_tournament_templates(&self) -> Result<Vec<TournamentTemplate>, AppError> {
        Ok(vec![
            TournamentTemplate {
                id: "single_elimination".to_string(),
                name: "Single Elimination".to_string(),
                description: "Standard single elimination tournament".to_string(),
                sport_type: SportType::Basketball,
                format: TournamentFormat::Elimination,
                default_settings: serde_json::json!({
                    "bracket_type": "single",
                    "seeding": "random",
                    "third_place": false
                }),
            },
            TournamentTemplate {
                id: "round_robin".to_string(),
                name: "Round Robin".to_string(),
                description: "Everyone plays everyone tournament format".to_string(),
                sport_type: SportType::TableTennis,
                format: TournamentFormat::RoundRobin,
                default_settings: serde_json::json!({
                    "points_win": 3,
                    "points_draw": 1,
                    "points_loss": 0
                }),
            },
        ])
    }

    pub async fn create_from_template(
        &self,
        _template_id: Uuid,
        mut data: NewTournament,
    ) -> Result<Tournament, AppError> {
        data.format = TournamentFormat::Elimination;
        self.tournament_repo.create(data).await
    }

    pub async fn get_tournament_dashboard(
        &self,
        id: Uuid,
    ) -> Result<TournamentDashboard, AppError> {
        let tournament = self
            .tournament_repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tournament not found".into()))?;
        let stats = self.tournament_repo.get_tournament_stats(id).await?;
        let categories = self.category_repo.get_by_tournament(id).await?;
        let all_registrations = self.registration_repo.get_by_tournament(id).await?;
        let recent_registrations = all_registrations.into_iter().take(10).collect();

        Ok(TournamentDashboard {
            tournament,
            stats,
            recent_registrations,
            categories,
        })
    }

    pub async fn update_tournament_settings(
        &self,
        id: Uuid,
        settings: JsonValue,
    ) -> Result<Option<Tournament>, AppError> {
        let data = EditableTournament {
            rules: Some(settings),
            ..Default::default()
        };
        self.tournament_repo.update(id, data).await
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

    /// Generate a default bracket for a tournament (single elimination, 0 rounds).
    pub async fn generate_bracket(
        &self,
        tournament_id: Uuid,
    ) -> Result<TournamentBracket, AppError> {
        let data = NewTournamentBracket {
            tournament_id,
            category_id: None,
            bracket_type: BracketType::SingleElimination,
            total_rounds: 0,
            bracket_data: None,
            settings: None,
        };
        self.bracket_repo.create(data).await
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
