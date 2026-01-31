use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

// ============ Enums (Value Objects) ============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Postponed,
    Forfeited,
    Bye,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    GroupStage,
    RoundOf128,
    RoundOf64,
    RoundOf32,
    RoundOf16,
    QuarterFinal,
    SemiFinal,
    ThirdPlace,
    Final,
    Qualifying,
    Playoff,
}

// ============ DTOs ============

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMatch {
    pub tournament_category_id: Uuid,
    pub participant1_team_id: Option<Uuid>,
    pub participant1_player_id: Option<Uuid>,
    pub participant1_partner_id: Option<Uuid>,
    pub participant2_team_id: Option<Uuid>,
    pub participant2_player_id: Option<Uuid>,
    pub participant2_partner_id: Option<Uuid>,
    pub match_type: MatchType,
    pub round_number: Option<i32>,
    pub match_number: Option<i32>,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditableMatch {
    pub match_status: Option<MatchStatus>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchStatusUpdate {
    pub match_status: MatchStatus,
    pub winner_participant: Option<i32>,
    pub is_draw: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchScheduleRequest {
    pub tournament_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub venue: Option<String>,
    pub status: Option<MatchStatus>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMatchRequest {
    pub tournament_category_id: Uuid,
    pub participant1_team_id: Option<Uuid>,
    pub participant1_player_id: Option<Uuid>,
    pub participant1_partner_id: Option<Uuid>,
    pub participant2_team_id: Option<Uuid>,
    pub participant2_player_id: Option<Uuid>,
    pub participant2_partner_id: Option<Uuid>,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub round_number: Option<i32>,
    pub match_number: Option<i32>,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMatchRequest {
    pub scheduled_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMatchStatusRequest {
    pub status: MatchStatus,
    pub winner_participant: Option<i32>,
    pub is_draw: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteMatchRequest {
    pub winner_participant: i32,
    pub is_draw: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelMatchRequest {
    pub reason: String,
    pub notify_participants: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostponeMatchRequest {
    pub new_scheduled_date: DateTime<Utc>,
    pub reason: String,
    pub notify_participants: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RescheduleMatchRequest {
    pub new_scheduled_date: DateTime<Utc>,
    pub new_venue: Option<String>,
    pub new_court_number: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveMatchUpdate {
    pub current_score: Option<JsonValue>,
    pub game_time: Option<i32>,
    pub current_set: Option<i32>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMatchCommentRequest {
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeToMatchRequest {
    pub notification_preferences: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkUpdateMatchesRequest {
    pub match_ids: Vec<Uuid>,
    pub updates: UpdateMatchRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkCancelMatchesRequest {
    pub match_ids: Vec<Uuid>,
    pub reason: String,
    pub notify_participants: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadMatchMediaRequest {
    pub media_type: String,
    pub file_name: String,
    pub file_size: i64,
    pub duration: Option<i32>,
}

// Match Result DTOs

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMatchResult {
    pub match_id: Uuid,
    pub set_number: Option<i32>,
    pub participant1_score: Option<i32>,
    pub participant2_score: Option<i32>,
    pub period_number: Option<i32>,
    pub period_name: Option<String>,
    pub scoring_data: Option<JsonValue>,
    pub participant1_stats: Option<JsonValue>,
    pub participant2_stats: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditableMatchResult {
    pub participant1_score: Option<i32>,
    pub participant2_score: Option<i32>,
    pub scoring_data: Option<JsonValue>,
    pub participant1_stats: Option<JsonValue>,
    pub participant2_stats: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchScoreSummary {
    pub match_id: Uuid,
    pub participant1_sets_won: i64,
    pub participant2_sets_won: i64,
    pub participant1_total_points: i64,
    pub participant2_total_points: i64,
}
