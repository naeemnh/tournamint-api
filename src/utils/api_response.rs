use crate::formatters;
use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use serde::Serialize;

pub struct ApiResponse;

#[allow(dead_code)]
impl ApiResponse {
    pub fn success<T: Serialize>(message: &str, data: Option<T>) -> HttpResponse {
        match data {
            Some(d) => formatters::success_response(StatusCode::OK, d, message),
            None => formatters::success_response(StatusCode::OK, serde_json::json!({}), message),
        }
    }

    pub fn created<T: Serialize>(message: &str, data: T) -> HttpResponse {
        formatters::success_response(StatusCode::CREATED, data, message)
    }

    pub fn error(message: &str) -> HttpResponse {
        formatters::error_response(StatusCode::INTERNAL_SERVER_ERROR, message, "INTERNAL_ERROR")
    }

    pub fn bad_request(message: &str) -> HttpResponse {
        formatters::error_response(StatusCode::BAD_REQUEST, message, "BAD_REQUEST")
    }

    pub fn not_found(message: &str) -> HttpResponse {
        formatters::error_response(StatusCode::NOT_FOUND, message, "NOT_FOUND")
    }

    pub fn unauthorized(message: &str) -> HttpResponse {
        formatters::error_response(StatusCode::UNAUTHORIZED, message, "UNAUTHORIZED")
    }

    pub fn forbidden(message: &str) -> HttpResponse {
        formatters::error_response(StatusCode::FORBIDDEN, message, "FORBIDDEN")
    }

    pub fn conflict(message: &str) -> HttpResponse {
        formatters::error_response(StatusCode::CONFLICT, message, "CONFLICT")
    }
}
