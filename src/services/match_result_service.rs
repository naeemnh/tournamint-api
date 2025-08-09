use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::match_result::{MatchResult, NewMatchResult, EditableMatchResult, MatchScoreSummary},
    repositories::match_result_repository::MatchResultRepository,
    utils::db::with_transaction,
};

pub struct MatchResultService;

impl MatchResultService {

    pub async fn create_match_result(
        pool: &DbPool,
        request: NewMatchResult,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::create(tx, request).await })
        })
        .await
        {
            Ok(match_result) => {
                formatters::success_response(StatusCode::CREATED, match_result, "MATCH_RESULT_CREATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("foreign key constraint") {
                    "Invalid match ID"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "MATCH_RESULT_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_match_result_by_id(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::find_by_id(tx, id).await })
        })
        .await
        {
            Ok(Some(match_result)) => {
                formatters::success_response(StatusCode::OK, match_result, "MATCH_RESULT_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Match result not found",
                "MATCH_RESULT_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "MATCH_RESULT_FETCH_ERROR",
            ),
        }
    }

    pub async fn update_match_result(
        pool: &DbPool,
        id: Uuid,
        request: EditableMatchResult,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::update(tx, id, request).await })
        })
        .await
        {
            Ok(match_result) => {
                formatters::success_response(StatusCode::OK, match_result, "MATCH_RESULT_UPDATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Match result not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "MATCH_RESULT_UPDATE_ERROR",
                )
            }
        }
    }

    pub async fn delete_match_result(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(_) => {
                formatters::success_response(StatusCode::OK, "Match result deleted", "MATCH_RESULT_DELETED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Match result not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "MATCH_RESULT_DELETE_ERROR",
                )
            }
        }
    }

    pub async fn get_match_results_by_match(pool: &DbPool, match_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::find_by_match(tx, match_id).await })
        })
        .await
        {
            Ok(match_results) => {
                formatters::success_response(StatusCode::OK, match_results, "MATCH_RESULTS_FOUND")
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "MATCH_RESULTS_FETCH_ERROR",
            ),
        }
    }

    pub async fn get_match_score_summary(pool: &DbPool, match_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::get_match_score_summary(tx, match_id).await })
        })
        .await
        {
            Ok(Some(summary)) => {
                formatters::success_response(StatusCode::OK, summary, "MATCH_SCORE_SUMMARY_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "No match results found for this match",
                "MATCH_SCORE_SUMMARY_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "MATCH_SCORE_SUMMARY_FETCH_ERROR",
            ),
        }
    }

    pub async fn get_match_results_by_set(
        pool: &DbPool,
        match_id: Uuid,
        set_number: i32,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::find_by_set(tx, match_id, set_number).await })
        })
        .await
        {
            Ok(match_results) => {
                formatters::success_response(StatusCode::OK, match_results, "MATCH_RESULTS_BY_SET_FOUND")
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "MATCH_RESULTS_BY_SET_FETCH_ERROR",
            ),
        }
    }

    pub async fn delete_all_match_results(pool: &DbPool, match_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::delete_by_match(tx, match_id).await })
        })
        .await
        {
            Ok(_) => {
                formatters::success_response(
                    StatusCode::OK,
                    "All match results deleted",
                    "ALL_MATCH_RESULTS_DELETED"
                )
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "ALL_MATCH_RESULTS_DELETE_ERROR",
            ),
        }
    }

    pub async fn count_match_results(pool: &DbPool, match_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { MatchResultRepository::count_by_match(tx, match_id).await })
        })
        .await
        {
            Ok(count) => {
                formatters::success_response(StatusCode::OK, count, "MATCH_RESULTS_COUNT_FOUND")
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "MATCH_RESULTS_COUNT_ERROR",
            ),
        }
    }

    pub async fn bulk_create_match_results(
        pool: &DbPool,
        requests: Vec<NewMatchResult>,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                let mut results = Vec::new();
                for request in requests {
                    let result = MatchResultRepository::create(tx, request).await?;
                    results.push(result);
                }
                Ok(results)
            })
        })
        .await
        {
            Ok(match_results) => {
                formatters::success_response(StatusCode::CREATED, match_results, "BULK_MATCH_RESULTS_CREATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("foreign key constraint") {
                    "Invalid match ID in bulk operation"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "BULK_MATCH_RESULTS_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn validate_match_result_scores(pool: &DbPool, match_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                let results = MatchResultRepository::find_by_match(tx, match_id).await?;
                
                // Basic validation: ensure scores are non-negative where present
                for result in results {
                    if let Some(score1) = result.participant1_score {
                        if score1 < 0 {
                            return Ok(false);
                        }
                    }
                    if let Some(score2) = result.participant2_score {
                        if score2 < 0 {
                            return Ok(false);
                        }
                    }
                }
                
                Ok(true)
            })
        })
        .await
        {
            Ok(is_valid) => {
                formatters::success_response(StatusCode::OK, is_valid, "MATCH_RESULT_SCORES_VALIDATED")
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "MATCH_RESULT_SCORES_VALIDATION_ERROR",
            ),
        }
    }
}