use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::value_objects::{MatchStatus, MatchType};

/// Core match entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    pub id: Uuid,
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
    pub actual_start_date: Option<DateTime<Utc>>,
    pub actual_end_date: Option<DateTime<Utc>>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub winner_participant: Option<i32>,
    pub is_draw: bool,
    pub referee_name: Option<String>,
    pub umpire_name: Option<String>,
    pub notes: Option<String>,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Match with resolved participant names for display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchWithParticipants {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub participant1_name: String,
    pub participant2_name: String,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub winner_participant: Option<i32>,
}

/// Match schedule item with tournament context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchScheduleItem {
    pub id: Uuid,
    pub tournament_category_id: Uuid,
    pub tournament_name: String,
    pub category_name: String,
    pub participant1_name: String,
    pub participant2_name: String,
    pub match_type: MatchType,
    pub match_status: MatchStatus,
    pub scheduled_date: DateTime<Utc>,
    pub venue: Option<String>,
    pub court_number: Option<String>,
    pub round_number: Option<i32>,
}

/// Match result/score entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub id: Uuid,
    pub match_id: Uuid,
    pub set_number: Option<i32>,
    pub participant1_score: Option<i32>,
    pub participant2_score: Option<i32>,
    pub period_number: Option<i32>,
    pub period_name: Option<String>,
    pub scoring_data: Option<JsonValue>,
    pub participant1_stats: Option<JsonValue>,
    pub participant2_stats: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Comment on a match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchComment {
    pub id: Uuid,
    pub match_id: Uuid,
    pub user_id: Uuid,
    pub comment: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// User subscription to match notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSubscription {
    pub id: Uuid,
    pub match_id: Uuid,
    pub user_id: Uuid,
    pub notification_preferences: JsonValue,
    pub created_at: DateTime<Utc>,
}

/// Match analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchAnalytics {
    pub match_id: Uuid,
    pub total_duration_minutes: Option<i32>,
    pub sets_played: Option<i32>,
    pub participant1_score: Option<JsonValue>,
    pub participant2_score: Option<JsonValue>,
    pub rally_stats: Option<JsonValue>,
    pub performance_metrics: Option<JsonValue>,
}

/// Match statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStatistics {
    pub match_id: Uuid,
    pub statistics: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Media attached to a match (video/photo)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchMedia {
    pub id: Uuid,
    pub match_id: Uuid,
    pub media_type: String, // "video" or "photo"
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}
