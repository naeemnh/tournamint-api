use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T: Serialize> {
    result: T,
    meta: String,
}

#[derive(Serialize)]
pub struct ErrorResponse<T: Serialize> {
    error: T,
    meta: String,
}

pub fn success_response<T: Serialize>(
    status: StatusCode,
    result: T,
    meta: impl Into<String>,
) -> HttpResponse {
    HttpResponse::build(status).json(SuccessResponse {
        result,
        meta: meta.into(),
    })
}

pub fn error_response<T: Serialize>(
    status: StatusCode,
    error: T,
    meta: impl Into<String>,
) -> HttpResponse {
    HttpResponse::build(status).json(ErrorResponse {
        error,
        meta: meta.into(),
    })
}
