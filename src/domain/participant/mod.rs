// Participant domain module - core business rules for players and teams

pub mod entity;
pub mod repository;
pub mod value_objects;

pub use entity::{Player, Team, TeamMember, TeamPlayer, TeamWithMembers};
pub use repository::{PlayerRepository, TeamMemberRepository, TeamRepository};
pub use value_objects::{CreatePlayer, EditablePlayer, EditableTeam, EditableTeamMember, NewTeam, NewTeamMember};
