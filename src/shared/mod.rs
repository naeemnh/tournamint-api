// Shared module - cross-cutting concerns used across all layers

pub mod api_response;
pub mod config;
pub mod errors;
pub mod google;
pub mod jwt;
pub mod types;

// Re-export commonly used items
pub use api_response::ApiResponse;
pub use config::{AppConfig, EnvConfig};
pub use errors::AppError;
