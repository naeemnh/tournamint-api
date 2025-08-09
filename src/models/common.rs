use serde::Deserialize;

/// Common pagination query parameters used across multiple endpoints
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    /// Maximum number of items to return (default: 50, max: 100)
    pub limit: Option<i64>,
    /// Number of items to skip for pagination
    pub offset: Option<i64>,
}

impl PaginationQuery {
    /// Get the limit value with a default and maximum cap
    pub fn get_limit(&self, default: i64, max: i64) -> i64 {
        self.limit.unwrap_or(default).min(max)
    }

    /// Get the offset value with a default of 0
    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }
}

/// Common date range query parameters
#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub from_date: Option<chrono::DateTime<chrono::Utc>>,
    pub to_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// Common response wrapper for paginated results
#[derive(Debug, serde::Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(serde::Deserialize)]
pub struct LeaderboardQueryParams {
    pub category: Option<String>, // "points", "wins", "earnings", "win_rate"
    pub entity_type: Option<String>, // "player", "team"
    pub sport_type: Option<String>, // Sport filter
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct RecordsQuery {
    pub limit: Option<i64>,
}
