use crate::models::match_model::{
    CreateMatchRequest, UpdateMatchRequest, UpdateMatchStatusRequest,
    CompleteMatchRequest, CancelMatchRequest, PostponeMatchRequest,
    RescheduleMatchRequest, LiveMatchUpdate, AddMatchCommentRequest,
    SubscribeToMatchRequest, BulkUpdateMatchesRequest, BulkCancelMatchesRequest,
    UploadMatchMediaRequest,
};
use crate::services::MatchService;
use crate::utils::api_response::ApiResponse;
use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use uuid::Uuid;
use serde_json::json;

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

    // 1. Get match with participants
    pub async fn get_match_with_participants(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.get_match_with_participants(match_id).await {
            Ok(Some(match_data)) => {
                ApiResponse::success("Match with participants retrieved successfully", Some(match_data))
            }
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 2. Start match
    pub async fn start_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.start_match(match_id).await {
            Ok(match_data) => {
                ApiResponse::success("Match started successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 3. Complete match
    pub async fn complete_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<CompleteMatchRequest>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.complete_match(match_id, payload.into_inner()).await {
            Ok(match_data) => {
                ApiResponse::success("Match completed successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 4. Cancel match
    pub async fn cancel_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<CancelMatchRequest>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.cancel_match(match_id, payload.into_inner()).await {
            Ok(match_data) => {
                ApiResponse::success("Match cancelled successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 5. Postpone match
    pub async fn postpone_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<PostponeMatchRequest>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.postpone_match(match_id, payload.into_inner()).await {
            Ok(match_data) => {
                ApiResponse::success("Match postponed successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 6. Get my upcoming matches
    pub async fn get_my_upcoming_matches(
        match_service: web::Data<MatchService>,
        req: HttpRequest,
    ) -> HttpResponse {
        // Extract user_id from authentication context
        // This assumes user_id is available in request extensions or headers
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            match match_service.get_user_upcoming_matches(*user_id).await {
                Ok(matches) => {
                    ApiResponse::success("Upcoming matches retrieved successfully", Some(matches))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 7. Get my match history
    pub async fn get_my_match_history(
        match_service: web::Data<MatchService>,
        req: HttpRequest,
    ) -> HttpResponse {
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            match match_service.get_user_match_history(*user_id).await {
                Ok(matches) => {
                    ApiResponse::success("Match history retrieved successfully", Some(matches))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 8. Reschedule match
    pub async fn reschedule_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<RescheduleMatchRequest>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.reschedule_match(match_id, payload.into_inner()).await {
            Ok(match_data) => {
                ApiResponse::success("Match rescheduled successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 9. Get live matches
    pub async fn get_live_matches(
        match_service: web::Data<MatchService>,
    ) -> HttpResponse {
        match match_service.get_live_matches().await {
            Ok(matches) => {
                ApiResponse::success("Live matches retrieved successfully", Some(matches))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 10. Update live match
    pub async fn update_match_live(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<LiveMatchUpdate>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.update_live_match(match_id, payload.into_inner()).await {
            Ok(match_data) => {
                ApiResponse::success("Live match updated successfully", Some(match_data))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 11. Get match analytics
    pub async fn get_match_analytics(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.get_match_analytics(match_id).await {
            Ok(analytics) => {
                ApiResponse::success("Match analytics retrieved successfully", Some(analytics))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 12. Get match statistics
    pub async fn get_match_statistics(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.get_match_statistics(match_id).await {
            Ok(statistics) => {
                ApiResponse::success("Match statistics retrieved successfully", Some(statistics))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 13. Get match media
    pub async fn get_match_media(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.get_match_media(match_id).await {
            Ok(media) => {
                ApiResponse::success("Match media retrieved successfully", Some(media))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 14. Upload match video
    pub async fn upload_match_video(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<UploadMatchMediaRequest>,
        req: HttpRequest,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            let mut media_request = payload.into_inner();
            media_request.media_type = "video".to_string();
            
            match match_service.upload_match_media(match_id, *user_id, media_request).await {
                Ok(media) => {
                    ApiResponse::success("Match video uploaded successfully", Some(media))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 15. Upload match photo
    pub async fn upload_match_photo(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<UploadMatchMediaRequest>,
        req: HttpRequest,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            let mut media_request = payload.into_inner();
            media_request.media_type = "photo".to_string();
            
            match match_service.upload_match_media(match_id, *user_id, media_request).await {
                Ok(media) => {
                    ApiResponse::success("Match photo uploaded successfully", Some(media))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 16. Get match comments
    pub async fn get_match_comments(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        match match_service.get_match_comments(match_id).await {
            Ok(comments) => {
                ApiResponse::success("Match comments retrieved successfully", Some(comments))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 17. Add match comment
    pub async fn add_match_comment(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<AddMatchCommentRequest>,
        req: HttpRequest,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            match match_service.add_match_comment(match_id, *user_id, payload.into_inner()).await {
                Ok(comment) => {
                    ApiResponse::success("Comment added successfully", Some(comment))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 18. Subscribe to match
    pub async fn subscribe_to_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        payload: web::Json<SubscribeToMatchRequest>,
        req: HttpRequest,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            match match_service.subscribe_to_match(match_id, *user_id, payload.into_inner()).await {
                Ok(subscription) => {
                    ApiResponse::success("Subscribed to match successfully", Some(subscription))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 19. Unsubscribe from match
    pub async fn unsubscribe_from_match(
        match_service: web::Data<MatchService>,
        path: web::Path<Uuid>,
        req: HttpRequest,
    ) -> HttpResponse {
        let match_id = path.into_inner();
        if let Some(user_id) = req.extensions().get::<Uuid>() {
            match match_service.unsubscribe_from_match(match_id, *user_id).await {
                Ok(_) => {
                    ApiResponse::success("Unsubscribed from match successfully", Some(json!({ "success": true })))
                }
                Err(e) => ApiResponse::error(&e.to_string()),
            }
        } else {
            ApiResponse::unauthorized("User authentication required")
        }
    }

    // 20. Bulk update matches
    pub async fn bulk_update_matches(
        match_service: web::Data<MatchService>,
        payload: web::Json<BulkUpdateMatchesRequest>,
    ) -> HttpResponse {
        match match_service.bulk_update_matches(payload.into_inner()).await {
            Ok(updated_matches) => {
                ApiResponse::success("Matches updated successfully", Some(updated_matches))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }

    // 21. Bulk cancel matches
    pub async fn bulk_cancel_matches(
        match_service: web::Data<MatchService>,
        payload: web::Json<BulkCancelMatchesRequest>,
    ) -> HttpResponse {
        match match_service.bulk_cancel_matches(payload.into_inner()).await {
            Ok(cancelled_matches) => {
                ApiResponse::success("Matches cancelled successfully", Some(cancelled_matches))
            }
            Err(e) => ApiResponse::error(&e.to_string()),
        }
    }
}
