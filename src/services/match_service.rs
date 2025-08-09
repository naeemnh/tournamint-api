use crate::models::match_model::{
    AddMatchCommentRequest, BulkCancelMatchesRequest, BulkUpdateMatchesRequest, CancelMatchRequest,
    CompleteMatchRequest, CreateMatchRequest, LiveMatchUpdate, Match, MatchAnalytics, MatchComment,
    MatchMedia, MatchStatistics, MatchSubscription, MatchWithParticipants, PostponeMatchRequest,
    RescheduleMatchRequest, SubscribeToMatchRequest, UpdateMatchRequest, UpdateMatchStatusRequest,
    UploadMatchMediaRequest,
};
use crate::repositories::match_repository::MatchRepository;
use anyhow::Result;
use uuid::Uuid;

pub struct MatchService {
    match_repository: MatchRepository,
}

impl MatchService {
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
        self.match_repository
            .find_by_tournament(tournament_id)
            .await
    }

    pub async fn get_matches_by_category(&self, category_id: Uuid) -> Result<Vec<Match>> {
        self.match_repository.find_by_category(category_id).await
    }

    pub async fn update_match_status(
        &self,
        id: Uuid,
        request: UpdateMatchStatusRequest,
    ) -> Result<Match> {
        self.match_repository.update_status(id, request).await
    }

    pub async fn get_match_schedule(&self) -> Result<Vec<Match>> {
        self.match_repository.find_scheduled().await
    }

    // Additional methods for the 21 missing controller methods

    pub async fn get_match_with_participants(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchWithParticipants>> {
        self.match_repository.find_with_participants(match_id).await
    }

    pub async fn start_match(&self, match_id: Uuid) -> Result<Match> {
        self.match_repository.start_match(match_id).await
    }

    pub async fn complete_match(
        &self,
        match_id: Uuid,
        request: CompleteMatchRequest,
    ) -> Result<Match> {
        self.match_repository
            .complete_match(match_id, request)
            .await
    }

    pub async fn cancel_match(&self, match_id: Uuid, request: CancelMatchRequest) -> Result<Match> {
        self.match_repository.cancel_match(match_id, request).await
    }

    pub async fn postpone_match(
        &self,
        match_id: Uuid,
        request: PostponeMatchRequest,
    ) -> Result<Match> {
        self.match_repository
            .postpone_match(match_id, request)
            .await
    }

    pub async fn get_user_upcoming_matches(&self, user_id: Uuid) -> Result<Vec<Match>> {
        self.match_repository
            .find_user_upcoming_matches(user_id)
            .await
    }

    pub async fn get_user_match_history(&self, user_id: Uuid) -> Result<Vec<Match>> {
        self.match_repository.find_user_match_history(user_id).await
    }

    pub async fn reschedule_match(
        &self,
        match_id: Uuid,
        request: RescheduleMatchRequest,
    ) -> Result<Match> {
        self.match_repository
            .reschedule_match(match_id, request)
            .await
    }

    pub async fn get_live_matches(&self) -> Result<Vec<Match>> {
        self.match_repository.find_live_matches().await
    }

    pub async fn update_live_match(
        &self,
        match_id: Uuid,
        update: LiveMatchUpdate,
    ) -> Result<Match> {
        self.match_repository
            .update_live_match(match_id, update)
            .await
    }

    pub async fn get_match_analytics(&self, match_id: Uuid) -> Result<MatchAnalytics> {
        self.match_repository.get_match_analytics(match_id).await
    }

    pub async fn get_match_statistics(&self, match_id: Uuid) -> Result<MatchStatistics> {
        self.match_repository.get_match_statistics(match_id).await
    }

    pub async fn get_match_media(&self, match_id: Uuid) -> Result<Vec<MatchMedia>> {
        self.match_repository.get_match_media(match_id).await
    }

    pub async fn upload_match_media(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        request: UploadMatchMediaRequest,
    ) -> Result<MatchMedia> {
        self.match_repository
            .upload_match_media(match_id, user_id, request)
            .await
    }

    pub async fn get_match_comments(&self, match_id: Uuid) -> Result<Vec<MatchComment>> {
        self.match_repository.get_match_comments(match_id).await
    }

    pub async fn add_match_comment(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        request: AddMatchCommentRequest,
    ) -> Result<MatchComment> {
        self.match_repository
            .add_match_comment(match_id, user_id, request)
            .await
    }

    pub async fn subscribe_to_match(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        request: SubscribeToMatchRequest,
    ) -> Result<MatchSubscription> {
        self.match_repository
            .subscribe_to_match(match_id, user_id, request)
            .await
    }

    pub async fn unsubscribe_from_match(&self, match_id: Uuid, user_id: Uuid) -> Result<()> {
        self.match_repository
            .unsubscribe_from_match(match_id, user_id)
            .await
    }

    pub async fn bulk_update_matches(
        &self,
        request: BulkUpdateMatchesRequest,
    ) -> Result<Vec<Match>> {
        self.match_repository.bulk_update_matches(request).await
    }

    pub async fn bulk_cancel_matches(
        &self,
        request: BulkCancelMatchesRequest,
    ) -> Result<Vec<Match>> {
        self.match_repository.bulk_cancel_matches(request).await
    }
}
