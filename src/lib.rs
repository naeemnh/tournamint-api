// Server library exports
// This file exports the modules needed for testing

// ==================== NEW DDD ARCHITECTURE ====================
// Domain-Driven Design layers:
// - domain: Core business rules (no external dependencies)
// - application: Use cases (orchestrates domain logic)
// - infra: External implementations (DB, API)
// - shared: Cross-cutting concerns

pub mod shared;
pub mod domain;
pub mod application;
pub mod infra;