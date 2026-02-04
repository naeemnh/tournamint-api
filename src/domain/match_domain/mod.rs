// Match domain module - core business rules for match management
// Note: "match" is a reserved keyword in Rust, hence "match_domain"

pub mod entity;
pub mod repository;
pub mod value_objects;

pub use entity::{
    Match, MatchAnalytics, MatchComment, MatchMedia, MatchResult, MatchScheduleItem,
    MatchStatistics, MatchSubscription, MatchWithParticipants,
};
pub use repository::{MatchRepository, MatchResultRepository};
pub use value_objects::{
    AddMatchCommentRequest, BulkCancelMatchesRequest, CancelMatchRequest, CompleteMatchRequest,
    EditableMatch, EditableMatchResult, LiveMatchUpdate, MatchScoreSummary, MatchStatus, MatchType,
    NewMatch, NewMatchResult, RescheduleMatchRequest, UpdateMatchStatusRequest,
};
