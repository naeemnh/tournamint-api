use anyhow::Result;
use uuid::Uuid;
use crate::models::match_model::{Match, CreateMatchRequest, UpdateMatchRequest, UpdateMatchStatusRequest};
use crate::repositories::match_repository::MatchRepository;

pub struct MatchService {
    match_repository: MatchRepository,
}

impl MatchService {
    pub fn new(match_repository: MatchRepository) -> Self {
        Self { match_repository }
    }

    pub async fn create_match(&self, request: CreateMatchRequest) -> Result<Match> {
        self.match_repository.create(request).await
    }

    pub async fn get_match_by_id(&self, id: Uuid) -> Result<Option<Match>> {
        self.match_repository.find_by_id(id).await
    }

    pub async fn update_match(&self, id: Uuid, request: UpdateMatchRequest) -> Result<Match> {
        self.match_repository.update(id, request).await
    }

    pub async fn delete_match(&self, id: Uuid) -> Result<()> {
        self.match_repository.delete(id).await
    }

    pub async fn get_matches_by_tournament(&self, tournament_id: Uuid) -> Result<Vec<Match>> {
        self.match_repository.find_by_tournament(tournament_id).await
    }

    pub async fn get_matches_by_category(&self, category_id: Uuid) -> Result<Vec<Match>> {
        self.match_repository.find_by_category(category_id).await
    }

    pub async fn update_match_status(&self, id: Uuid, request: UpdateMatchStatusRequest) -> Result<Match> {
        self.match_repository.update_status(id, request).await
    }

    pub async fn get_match_schedule(&self) -> Result<Vec<Match>> {
        self.match_repository.find_scheduled().await
    }
}