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

// ==================== LEGACY MODULES (for gradual migration) ====================
// These modules will be removed after migration is complete.
// The implementations will be migrated to the DDD structure above.

pub mod config;
pub mod constants;
pub mod controllers;
pub mod formatters;
pub mod middlewares;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod services;
pub mod utils;