use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::match_result::{NewMatchResult, EditableMatchResult},
    services::match_result_service::MatchResultService,
};

pub struct MatchResultController;

impl MatchResultController {
    pub async fn create_match_result(
        pool: web::Data<DbPool>,
        payload: web::Json<NewMatchResult>,
    ) -> HttpResponse {
        MatchResultService::create_match_result(&pool, payload.into_inner()).await
    }

    pub async fn get_match_result(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_result_id = path.into_inner();
        MatchResultService::get_match_result_by_id(&pool, match_result_id).await
    }

    pub async fn update_match_result(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
        payload: web::Json<EditableMatchResult>,
    ) -> HttpResponse {
        let match_result_id = path.into_inner();
        MatchResultService::update_match_result(&pool, match_result_id, payload.into_inner()).await
    }

    pub async fn delete_match_result(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_result_id = path.into_inner();
        MatchResultService::delete_match_result(&pool, match_result_id).await
    }

    pub async fn get_match_results_by_match(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        MatchResultService::get_match_results_by_match(&pool, match_id).await
    }

    pub async fn get_match_score_summary(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        MatchResultService::get_match_score_summary(&pool, match_id).await
    }

    pub async fn get_match_results_by_set(
        pool: web::Data<DbPool>,
        path: web::Path<(Uuid, i32)>,
    ) -> HttpResponse {
        let (match_id, set_number) = path.into_inner();
        MatchResultService::get_match_results_by_set(&pool, match_id, set_number).await
    }

    pub async fn delete_all_match_results(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        MatchResultService::delete_all_match_results(&pool, match_id).await
    }

    pub async fn bulk_create_match_results(
        pool: web::Data<DbPool>,
        payload: web::Json<Vec<NewMatchResult>>,
    ) -> HttpResponse {
        MatchResultService::bulk_create_match_results(&pool, payload.into_inner()).await
    }

    pub async fn validate_match_scores(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        MatchResultService::validate_match_result_scores(&pool, match_id).await
    }

    pub async fn count_match_results(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        MatchResultService::count_match_results(&pool, match_id).await
    }
}