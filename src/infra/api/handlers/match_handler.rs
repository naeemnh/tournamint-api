use actix_web::{web, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::application::MatchUseCases;
use crate::domain::match_domain::{
    EditableMatch, EditableMatchResult, NewMatch, NewMatchResult, RescheduleMatchRequest,
};
use crate::infra::db::{PgMatchRepository, PgMatchResultRepository};
use crate::shared::ApiResponse;

type MatchUseCasesData =
    std::sync::Arc<MatchUseCases<PgMatchRepository, PgMatchResultRepository>>;

pub struct MatchHandler;

impl MatchHandler {
    pub async fn post(
        use_cases: web::Data<MatchUseCasesData>,
        body: web::Json<NewMatch>,
    ) -> HttpResponse {
        match use_cases.create_match(body.into_inner()).await {
            Ok(m) => ApiResponse::created("Created", m),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match(id).await {
            Ok(Some(m)) => ApiResponse::success("OK", Some(m)),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableMatch>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_match(id, body.into_inner()).await {
            Ok(Some(m)) => ApiResponse::success("Updated", Some(m)),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_match(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn reschedule(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<RescheduleMatchRequest>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.reschedule_match(id, body.into_inner()).await {
            Ok(Some(m)) => ApiResponse::success("Match rescheduled", Some(m)),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn validate_result_scores(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.validate_match_result_scores(id).await {
            Ok(valid) => ApiResponse::success("OK", Some(serde_json::json!({ "valid": valid }))),
            Err(e) => e.error_response(),
        }
    }
}

pub struct MatchResultHandler;

impl MatchResultHandler {
    pub async fn post(
        use_cases: web::Data<MatchUseCasesData>,
        body: web::Json<NewMatchResult>,
    ) -> HttpResponse {
        match use_cases.create_match_result(body.into_inner()).await {
            Ok(r) => ApiResponse::created("Created", r),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match_result(id).await {
            Ok(Some(r)) => ApiResponse::success("OK", Some(r)),
            Ok(None) => ApiResponse::not_found("Match result not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableMatchResult>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_match_result(id, body.into_inner()).await {
            Ok(Some(r)) => ApiResponse::success("Updated", Some(r)),
            Ok(None) => ApiResponse::not_found("Match result not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn bulk_post(
        use_cases: web::Data<MatchUseCasesData>,
        body: web::Json<Vec<NewMatchResult>>,
    ) -> HttpResponse {
        match use_cases.bulk_create_match_results(body.into_inner()).await {
            Ok(results) => ApiResponse::created("Bulk match results created", results),
            Err(e) => e.error_response(),
        }
    }
}
