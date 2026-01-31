use async_trait::async_trait;
use uuid::Uuid;

use super::entity::{
    Match, MatchAnalytics, MatchComment, MatchMedia, MatchResult, MatchScheduleItem,
    MatchStatistics, MatchSubscription, MatchWithParticipants,
};
use super::value_objects::{
    EditableMatch, EditableMatchResult, LiveMatchUpdate, MatchScoreSummary, MatchStatus,
    NewMatch, NewMatchResult,
};
use crate::shared::AppError;

/// Repository trait for Match entity operations
#[async_trait]
pub trait MatchRepository: Send + Sync {
    // Basic CRUD
    async fn create(&self, new_match: NewMatch) -> Result<Match, AppError>;
    async fn find_by_id(&self, match_id: Uuid) -> Result<Option<Match>, AppError>;
    async fn update(&self, match_id: Uuid, match_data: EditableMatch) -> Result<Option<Match>, AppError>;
    async fn delete(&self, match_id: Uuid) -> Result<Option<Match>, AppError>;
    
    // Query methods
    async fn find_by_tournament(&self, tournament_id: Uuid) -> Result<Vec<Match>, AppError>;
    async fn find_by_category(&self, category_id: Uuid) -> Result<Vec<Match>, AppError>;
    async fn find_scheduled(&self) -> Result<Vec<MatchScheduleItem>, AppError>;
    async fn find_with_participants(&self, match_id: Uuid) -> Result<Option<MatchWithParticipants>, AppError>;
    
    // Status management
    async fn update_status(&self, match_id: Uuid, status: MatchStatus) -> Result<Option<Match>, AppError>;
    async fn start_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError>;
    async fn complete_match(&self, match_id: Uuid, winner: i32, is_draw: bool) -> Result<Option<Match>, AppError>;
    async fn cancel_match(&self, match_id: Uuid, reason: &str) -> Result<Option<Match>, AppError>;
    async fn postpone_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError>;
    
    // User-specific queries
    async fn find_user_upcoming_matches(&self, user_id: Uuid) -> Result<Vec<MatchScheduleItem>, AppError>;
    async fn find_user_match_history(&self, user_id: Uuid) -> Result<Vec<MatchScheduleItem>, AppError>;
    
    // Live match
    async fn find_live_matches(&self) -> Result<Vec<Match>, AppError>;
    async fn update_live_match(&self, match_id: Uuid, update: LiveMatchUpdate) -> Result<Option<Match>, AppError>;
    
    // Analytics
    async fn get_match_analytics(&self, match_id: Uuid) -> Result<Option<MatchAnalytics>, AppError>;
    async fn get_match_statistics(&self, match_id: Uuid) -> Result<Option<MatchStatistics>, AppError>;
    
    // Media
    async fn get_match_media(&self, match_id: Uuid) -> Result<Vec<MatchMedia>, AppError>;
    async fn upload_match_media(&self, match_id: Uuid, user_id: Uuid, media_type: &str, file_url: &str) -> Result<MatchMedia, AppError>;
    
    // Comments
    async fn get_match_comments(&self, match_id: Uuid) -> Result<Vec<MatchComment>, AppError>;
    async fn add_match_comment(&self, match_id: Uuid, user_id: Uuid, comment: &str) -> Result<MatchComment, AppError>;
    
    // Subscriptions
    async fn subscribe_to_match(&self, match_id: Uuid, user_id: Uuid) -> Result<MatchSubscription, AppError>;
    async fn unsubscribe_from_match(&self, match_id: Uuid, user_id: Uuid) -> Result<(), AppError>;
    
    // Bulk operations
    async fn bulk_update_matches(&self, match_ids: Vec<Uuid>, updates: EditableMatch) -> Result<Vec<Match>, AppError>;
    async fn bulk_cancel_matches(&self, match_ids: Vec<Uuid>, reason: &str) -> Result<Vec<Match>, AppError>;
}

/// Repository trait for MatchResult entity operations
#[async_trait]
pub trait MatchResultRepository: Send + Sync {
    async fn create(&self, new_result: NewMatchResult) -> Result<MatchResult, AppError>;
    async fn find_by_id(&self, result_id: Uuid) -> Result<Option<MatchResult>, AppError>;
    async fn update(&self, result_id: Uuid, result_data: EditableMatchResult) -> Result<Option<MatchResult>, AppError>;
    async fn delete(&self, result_id: Uuid) -> Result<Option<MatchResult>, AppError>;
    async fn find_by_match(&self, match_id: Uuid) -> Result<Vec<MatchResult>, AppError>;
    async fn get_match_score_summary(&self, match_id: Uuid) -> Result<Option<MatchScoreSummary>, AppError>;
    async fn find_by_set(&self, match_id: Uuid, set_number: i32) -> Result<Vec<MatchResult>, AppError>;
    async fn delete_by_match(&self, match_id: Uuid) -> Result<u64, AppError>;
    async fn count_by_match(&self, match_id: Uuid) -> Result<i64, AppError>;
}
