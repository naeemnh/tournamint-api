// Tournament domain module - core business rules for tournament management

pub mod entity;
pub mod repository;
pub mod value_objects;

pub use entity::{
    Tournament, TournamentBracket, TournamentCategory, TournamentRegistration, TournamentStandings,
    RegistrationWithDetails,
};
pub use repository::{
    TournamentBracketRepository, TournamentCategoryRepository, TournamentRegistrationRepository,
    TournamentRepository, TournamentStandingsRepository,
};
pub use value_objects::{
    BracketStatus, BracketType, EditableTournament, EditableTournamentBracket,
    EditableTournamentCategory, EditableTournamentRegistration, EditableTournamentStandings, NewTournament, NewTournamentBracket, NewTournamentCategory,
    NewTournamentRegistration, NewTournamentStandings, PaymentStatus,
    RegistrationStatus, SportType,
    TeamComposition, TournamentFormat, TournamentStatus,
};
