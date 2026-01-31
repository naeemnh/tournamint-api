use std::sync::Arc;
use uuid::Uuid;

use crate::domain::match_domain::{
    EditableMatch, EditableMatchResult, LiveMatchUpdate, Match, MatchAnalytics, MatchComment,
    MatchMedia, MatchRepository, MatchResult, MatchResultRepository, MatchScheduleItem,
    MatchScoreSummary, MatchStatistics, MatchStatus, MatchSubscription, MatchWithParticipants,
    NewMatch, NewMatchResult, RescheduleMatchRequest,
};
use crate::shared::AppError;

/// Match domain use cases
pub struct MatchUseCases<M, R>
where
    M: MatchRepository,
    R: MatchResultRepository,
{
    match_repo: Arc<M>,
    result_repo: Arc<R>,
}

impl<M, R> MatchUseCases<M, R>
where
    M: MatchRepository,
    R: MatchResultRepository,
{
    pub fn new(match_repo: Arc<M>, result_repo: Arc<R>) -> Self {
        Self {
            match_repo,
            result_repo,
        }
    }

    // ==================== Match CRUD ====================

    pub async fn create_match(&self, data: NewMatch) -> Result<Match, AppError> {
        self.match_repo.create(data).await
    }

    pub async fn get_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        self.match_repo.find_by_id(match_id).await
    }

    pub async fn update_match(
        &self,
        match_id: Uuid,
        data: EditableMatch,
    ) -> Result<Option<Match>, AppError> {
        self.match_repo.update(match_id, data).await
    }

    pub async fn delete_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        self.match_repo.delete(match_id).await
    }

    // ==================== Query ====================

    pub async fn get_matches_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<Match>, AppError> {
        self.match_repo.find_by_tournament(tournament_id).await
    }

    pub async fn get_matches_by_category(&self, category_id: Uuid) -> Result<Vec<Match>, AppError> {
        self.match_repo.find_by_category(category_id).await
    }

    pub async fn get_match_schedule(&self) -> Result<Vec<MatchScheduleItem>, AppError> {
        self.match_repo.find_scheduled().await
    }

    pub async fn get_match_with_participants(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchWithParticipants>, AppError> {
        self.match_repo.find_with_participants(match_id).await
    }

    // ==================== Status Management ====================

    pub async fn update_match_status(
        &self,
        match_id: Uuid,
        status: MatchStatus,
    ) -> Result<Option<Match>, AppError> {
        self.match_repo.update_status(match_id, status).await
    }

    pub async fn start_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        self.match_repo.start_match(match_id).await
    }

    pub async fn complete_match(
        &self,
        match_id: Uuid,
        winner: i32,
        is_draw: bool,
    ) -> Result<Option<Match>, AppError> {
        self.match_repo.complete_match(match_id, winner, is_draw).await
    }

    pub async fn cancel_match(
        &self,
        match_id: Uuid,
        reason: &str,
    ) -> Result<Option<Match>, AppError> {
        self.match_repo.cancel_match(match_id, reason).await
    }

    pub async fn postpone_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        self.match_repo.postpone_match(match_id).await
    }

    pub async fn reschedule_match(
        &self,
        match_id: Uuid,
        request: RescheduleMatchRequest,
    ) -> Result<Option<Match>, AppError> {
        self.match_repo.reschedule_match(match_id, request).await
    }

    // ==================== User-specific ====================

    pub async fn get_user_upcoming_matches(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MatchScheduleItem>, AppError> {
        self.match_repo.find_user_upcoming_matches(user_id).await
    }

    pub async fn get_user_match_history(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MatchScheduleItem>, AppError> {
        self.match_repo.find_user_match_history(user_id).await
    }

    // ==================== Live ====================

    pub async fn get_live_matches(&self) -> Result<Vec<Match>, AppError> {
        self.match_repo.find_live_matches().await
    }

    pub async fn update_live_match(
        &self,
        match_id: Uuid,
        update: LiveMatchUpdate,
    ) -> Result<Option<Match>, AppError> {
        self.match_repo.update_live_match(match_id, update).await
    }

    // ==================== Analytics ====================

    pub async fn get_match_analytics(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchAnalytics>, AppError> {
        self.match_repo.get_match_analytics(match_id).await
    }

    pub async fn get_match_statistics(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchStatistics>, AppError> {
        self.match_repo.get_match_statistics(match_id).await
    }

    // ==================== Media ====================

    pub async fn get_match_media(&self, match_id: Uuid) -> Result<Vec<MatchMedia>, AppError> {
        self.match_repo.get_match_media(match_id).await
    }

    pub async fn upload_match_media(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        media_type: &str,
        file_url: &str,
    ) -> Result<MatchMedia, AppError> {
        self.match_repo
            .upload_match_media(match_id, user_id, media_type, file_url)
            .await
    }

    // ==================== Comments ====================

    pub async fn get_match_comments(&self, match_id: Uuid) -> Result<Vec<MatchComment>, AppError> {
        self.match_repo.get_match_comments(match_id).await
    }

    pub async fn add_match_comment(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        comment: &str,
    ) -> Result<MatchComment, AppError> {
        self.match_repo
            .add_match_comment(match_id, user_id, comment)
            .await
    }

    // ==================== Subscriptions ====================

    pub async fn subscribe_to_match(
        &self,
        match_id: Uuid,
        user_id: Uuid,
    ) -> Result<MatchSubscription, AppError> {
        self.match_repo.subscribe_to_match(match_id, user_id).await
    }

    pub async fn unsubscribe_from_match(&self, match_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.match_repo.unsubscribe_from_match(match_id, user_id).await
    }

    // ==================== Bulk ====================

    pub async fn bulk_update_matches(
        &self,
        match_ids: Vec<Uuid>,
        updates: EditableMatch,
    ) -> Result<Vec<Match>, AppError> {
        self.match_repo.bulk_update_matches(match_ids, updates).await
    }

    pub async fn bulk_cancel_matches(
        &self,
        match_ids: Vec<Uuid>,
        reason: &str,
    ) -> Result<Vec<Match>, AppError> {
        self.match_repo.bulk_cancel_matches(match_ids, reason).await
    }

    // ==================== Match Results ====================

    pub async fn create_match_result(&self, data: NewMatchResult) -> Result<MatchResult, AppError> {
        self.result_repo.create(data).await
    }

    pub async fn get_match_result(&self, result_id: Uuid) -> Result<Option<MatchResult>, AppError> {
        self.result_repo.find_by_id(result_id).await
    }

    pub async fn update_match_result(
        &self,
        result_id: Uuid,
        data: EditableMatchResult,
    ) -> Result<Option<MatchResult>, AppError> {
        self.result_repo.update(result_id, data).await
    }

    pub async fn delete_match_result(
        &self,
        result_id: Uuid,
    ) -> Result<Option<MatchResult>, AppError> {
        self.result_repo.delete(result_id).await
    }

    pub async fn get_match_results(&self, match_id: Uuid) -> Result<Vec<MatchResult>, AppError> {
        self.result_repo.find_by_match(match_id).await
    }

    pub async fn get_match_score_summary(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchScoreSummary>, AppError> {
        self.result_repo.get_match_score_summary(match_id).await
    }

    pub async fn get_match_results_by_set(
        &self,
        match_id: Uuid,
        set_number: i32,
    ) -> Result<Vec<MatchResult>, AppError> {
        self.result_repo.find_by_set(match_id, set_number).await
    }

    pub async fn delete_all_match_results(&self, match_id: Uuid) -> Result<u64, AppError> {
        self.result_repo.delete_by_match(match_id).await
    }

    pub async fn count_match_results(&self, match_id: Uuid) -> Result<i64, AppError> {
        self.result_repo.count_by_match(match_id).await
    }

    pub async fn bulk_create_match_results(
        &self,
        items: Vec<NewMatchResult>,
    ) -> Result<Vec<MatchResult>, AppError> {
        let mut results = Vec::with_capacity(items.len());
        for item in items {
            let r = self.result_repo.create(item).await?;
            results.push(r);
        }
        Ok(results)
    }

    pub async fn validate_match_result_scores(&self, match_id: Uuid) -> Result<bool, AppError> {
        let results = self.result_repo.find_by_match(match_id).await?;
        for result in results {
            if let Some(score1) = result.participant1_score {
                if score1 < 0 {
                    return Ok(false);
                }
            }
            if let Some(score2) = result.participant2_score {
                if score2 < 0 {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
}
