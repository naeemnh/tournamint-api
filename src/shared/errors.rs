use actix_web::{HttpResponse, ResponseError};
use std::fmt;

// Database error constants
pub const DUPLICATE_USER_USERNAME: &str = "error returned from database: duplicate key value violates unique constraint \"users_username_key\"";
pub const DUPLICATE_USER_EMAIL: &str = "error returned from database: duplicate key value violates unique constraint \"users_email_key\"";

/// Application-wide error type
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    InternalError(String),
    DatabaseError(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        use crate::shared::api_response::ApiResponse;
        
        match self {
            AppError::NotFound(msg) => ApiResponse::not_found(msg),
            AppError::BadRequest(msg) => ApiResponse::bad_request(msg),
            AppError::Unauthorized(msg) => ApiResponse::unauthorized(msg),
            AppError::Forbidden(msg) => ApiResponse::forbidden(msg),
            AppError::Conflict(msg) => ApiResponse::conflict(msg),
            AppError::InternalError(msg) => ApiResponse::error(msg),
            AppError::DatabaseError(msg) => ApiResponse::error(msg),
            AppError::ValidationError(msg) => ApiResponse::bad_request(msg),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
