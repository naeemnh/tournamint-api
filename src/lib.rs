// Server library exports
// This file exports the modules needed for testing

// ==================== NEW DDD ARCHITECTURE ====================
// Domain-Driven Design layers:
// - domain: Core business rules (no external dependencies)
// - application: Services (orchestrates domain logic)
// - infra: External implementations (DB, API)
// - shared: Cross-cutting concerns

pub mod application;
pub mod domain;
pub mod infra;
pub mod shared;
