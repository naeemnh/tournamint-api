use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
struct SuccessResponse<T: Serialize> {
    result: T,
    meta: String,
}

#[derive(Serialize)]
struct ErrorResponse<T: Serialize> {
    error: T,
    meta: String,
}

fn success_response<T: Serialize>(
    status: StatusCode,
    result: T,
    meta: impl Into<String>,
) -> HttpResponse {
    HttpResponse::build(status).json(SuccessResponse {
        result,
        meta: meta.into(),
    })
}

fn error_response<T: Serialize>(
    status: StatusCode,
    error: T,
    meta: impl Into<String>,
) -> HttpResponse {
    HttpResponse::build(status).json(ErrorResponse {
        error,
        meta: meta.into(),
    })
}

pub struct ApiResponse;

#[allow(dead_code)]
impl ApiResponse {
    pub fn success<T: Serialize>(message: &str, data: Option<T>) -> HttpResponse {
        match data {
            Some(d) => success_response(StatusCode::OK, d, message),
            None => success_response(StatusCode::OK, serde_json::json!({}), message),
        }
    }

    pub fn created<T: Serialize>(message: &str, data: T) -> HttpResponse {
        success_response(StatusCode::CREATED, data, message)
    }

    pub fn error(message: &str) -> HttpResponse {
        error_response(StatusCode::INTERNAL_SERVER_ERROR, message, "INTERNAL_ERROR")
    }

    pub fn bad_request(message: &str) -> HttpResponse {
        error_response(StatusCode::BAD_REQUEST, message, "BAD_REQUEST")
    }

    pub fn not_found(message: &str) -> HttpResponse {
        error_response(StatusCode::NOT_FOUND, message, "NOT_FOUND")
    }

    pub fn unauthorized(message: &str) -> HttpResponse {
        error_response(StatusCode::UNAUTHORIZED, message, "UNAUTHORIZED")
    }

    pub fn forbidden(message: &str) -> HttpResponse {
        error_response(StatusCode::FORBIDDEN, message, "FORBIDDEN")
    }

    pub fn conflict(message: &str) -> HttpResponse {
        error_response(StatusCode::CONFLICT, message, "CONFLICT")
    }
}
