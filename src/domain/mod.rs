// Domain layer - core business rules (no external dependencies)

pub mod match_domain;
pub mod notification;
pub mod participant;
pub mod payment;
pub mod statistics;
pub mod tournament;
pub mod user;

// Re-export domain modules for convenient access
