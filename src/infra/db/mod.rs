// Database infrastructure - repository implementations

pub mod pool;

pub mod match_repo;
pub mod match_result_repo;
pub mod notification_repo;
pub mod payment_repo;
pub mod player_repo;
pub mod statistics_repo;
pub mod team_member_repo;
pub mod team_repo;
pub mod tournament_bracket_repo;
pub mod tournament_category_repo;
pub mod tournament_registration_repo;
pub mod tournament_repo;
pub mod tournament_standings_repo;
pub mod user_repo;

// Re-exports
pub use match_repo::PgMatchRepository;
pub use match_result_repo::PgMatchResultRepository;
pub use notification_repo::PgNotificationRepository;
pub use payment_repo::PgPaymentRepository;
pub use player_repo::PgPlayerRepository;
pub use pool::DbConfig;
pub use statistics_repo::PgStatisticsRepository;
pub use team_member_repo::PgTeamMemberRepository;
pub use team_repo::PgTeamRepository;
pub use tournament_bracket_repo::PgTournamentBracketRepository;
pub use tournament_category_repo::PgTournamentCategoryRepository;
pub use tournament_registration_repo::PgTournamentRegistrationRepository;
pub use tournament_repo::PgTournamentRepository;
pub use tournament_standings_repo::PgTournamentStandingsRepository;
pub use user_repo::{PgTokenRepository, PgUserProfileRepository, PgUserRepository};
