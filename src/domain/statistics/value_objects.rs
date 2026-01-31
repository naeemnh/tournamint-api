use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticsFilters {
    pub sport_type: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub tournament_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardRequest {
    pub category: String, // "points", "wins", "earnings", "win_rate"
    pub entity_type: String, // "player" or "team"
    pub sport_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
