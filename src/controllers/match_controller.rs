use crate::models::match_model::{
    CreateMatchRequest, UpdateMatchRequest, UpdateMatchStatusRequest,
};
use crate::services::MatchService;
use crate::utils::api_response::ApiResponse;
use actix_web::{web, HttpResponse};
use uuid::Uuid;

pub struct MatchController;

impl MatchController {
    pub async fn create_match(
        match_service: web::Data<MatchService>,
        payload: web::Json<CreateMatchRequest>,
    ) -> HttpResponse {
        match match_service.create_match(payload.into_inner()).await {
            Ok(match_data) => ApiResponse::success("Match created successfully", Some(match_data)),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn get_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.get_match_by_id(match_id).await {
            Ok(Some(match_data)) => {
                ApiResponse::success("Match retrieved successfully", Some(match_data))
            }
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn update_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<UpdateMatchRequest>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service
            .update_match(match_id, payload.into_inner())
            .await
        {
            Ok(match_data) => ApiResponse::success("Match updated successfully", Some(match_data)),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn delete_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.delete_match(match_id).await {
            Ok(_) => ApiResponse::success("Match deleted successfully", None::<()>),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn get_matches_by_tournament(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let tournament_id = path.into_inner();
        match match_service.get_matches_by_tournament(tournament_id).await {
            Ok(matches) => ApiResponse::success("Matches retrieved successfully", Some(matches)),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn get_matches_by_category(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let category_id = path.into_inner();
        match match_service.get_matches_by_category(category_id).await {
            Ok(matches) => ApiResponse::success("Matches retrieved successfully", Some(matches)),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn update_match_status(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<UpdateMatchStatusRequest>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service
            .update_match_status(match_id, payload.into_inner())
            .await
        {
            Ok(match_data) => {
                ApiResponse::success("Match status updated successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    pub async fn get_match_schedule(match_service: web::Data<MatchService>) -> HttpResponse {
        match match_service.get_match_schedule().await {
            Ok(matches) => {
                ApiResponse::success("Match schedule retrieved successfully", Some(matches))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }
}
