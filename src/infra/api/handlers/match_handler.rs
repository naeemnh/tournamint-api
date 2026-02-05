use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::MatchUseCases;
use crate::domain::match_domain::{
    AddMatchCommentRequest, BulkCancelMatchesRequest, CancelMatchRequest, CompleteMatchRequest,
    EditableMatch, EditableMatchResult, LiveMatchUpdate, NewMatch, NewMatchResult,
    RescheduleMatchRequest, UpdateMatchStatusRequest,
};
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::api::sse::{Broadcaster, RealtimeEvent};
use crate::infra::api::multipart_util::extract_file_from_multipart;
use crate::infra::cloudinary::CloudinaryClient;
use crate::infra::db::{PgMatchRepository, PgMatchResultRepository};
use crate::shared::ApiResponse;

type MatchUseCasesData =
    std::sync::Arc<MatchUseCases<PgMatchRepository, PgMatchResultRepository>>;

#[derive(Debug, Deserialize)]
pub struct TournamentIdPath {
    pub tournament_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CategoryIdPath {
    pub category_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct MatchIdPath {
    pub match_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct MatchSetPath {
    pub match_id: Uuid,
    pub set_number: i32,
}

/// Body for PUT /matches/bulk/update: match_ids + EditableMatch
#[derive(Debug, Deserialize)]
pub struct BulkMatchUpdateBody {
    pub match_ids: Vec<Uuid>,
    pub updates: EditableMatch,
}

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

    pub async fn get_by_tournament(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases.get_matches_by_tournament(path.tournament_id).await {
            Ok(matches) => ApiResponse::success("OK", Some(matches)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_category(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match use_cases.get_matches_by_category(path.category_id).await {
            Ok(matches) => ApiResponse::success("OK", Some(matches)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_participants(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match_with_participants(id).await {
            Ok(Some(m)) => ApiResponse::success("OK", Some(m)),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_status(
        use_cases: web::Data<MatchUseCasesData>,
        broadcaster: web::Data<std::sync::Arc<Broadcaster>>,
        path: web::Path<Uuid>,
        body: web::Json<UpdateMatchStatusRequest>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_match_status(id, body.status).await {
            Ok(Some(m)) => {
                let status = format!("{:?}", m.match_status);
                broadcaster
                    .broadcast_event(&RealtimeEvent::MatchUpdate {
                        match_id: m.id,
                        tournament_id: None,
                        category_id: Some(m.tournament_category_id),
                        status: Some(status),
                    })
                    .await;
                ApiResponse::success("Updated", Some(m))
            }
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn start(
        use_cases: web::Data<MatchUseCasesData>,
        broadcaster: web::Data<std::sync::Arc<Broadcaster>>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.start_match(id).await {
            Ok(Some(m)) => {
                broadcaster
                    .broadcast_event(&RealtimeEvent::MatchUpdate {
                        match_id: m.id,
                        tournament_id: None,
                        category_id: Some(m.tournament_category_id),
                        status: Some("InProgress".to_string()),
                    })
                    .await;
                ApiResponse::success("Started", Some(m))
            }
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn complete(
        use_cases: web::Data<MatchUseCasesData>,
        broadcaster: web::Data<std::sync::Arc<Broadcaster>>,
        path: web::Path<Uuid>,
        body: web::Json<CompleteMatchRequest>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.complete_match(id, body.winner_participant, body.is_draw).await {
            Ok(Some(m)) => {
                broadcaster
                    .broadcast_event(&RealtimeEvent::MatchUpdate {
                        match_id: m.id,
                        tournament_id: None,
                        category_id: Some(m.tournament_category_id),
                        status: Some("Completed".to_string()),
                    })
                    .await;
                ApiResponse::success("Completed", Some(m))
            }
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn cancel(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
        body: Option<web::Json<CancelMatchRequest>>,
    ) -> HttpResponse {
        let id = path.into_inner();
        let reason = body.as_ref().map(|b| b.reason.as_str()).unwrap_or("");
        match use_cases.cancel_match(id, reason).await {
            Ok(Some(m)) => ApiResponse::success("Cancelled", Some(m)),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn postpone(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.postpone_match(id).await {
            Ok(Some(m)) => ApiResponse::success("Postponed", Some(m)),
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_schedule(use_cases: web::Data<MatchUseCasesData>) -> HttpResponse {
        match use_cases.get_match_schedule().await {
            Ok(items) => ApiResponse::success("OK", Some(items)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn my_upcoming(
        use_cases: web::Data<MatchUseCasesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.get_user_upcoming_matches(user_id).await {
            Ok(items) => ApiResponse::success("OK", Some(items)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn my_history(
        use_cases: web::Data<MatchUseCasesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.get_user_match_history(user_id).await {
            Ok(items) => ApiResponse::success("OK", Some(items)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_live(use_cases: web::Data<MatchUseCasesData>) -> HttpResponse {
        match use_cases.get_live_matches().await {
            Ok(matches) => ApiResponse::success("OK", Some(matches)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_live(
        use_cases: web::Data<MatchUseCasesData>,
        broadcaster: web::Data<std::sync::Arc<Broadcaster>>,
        path: web::Path<Uuid>,
        body: web::Json<LiveMatchUpdate>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_live_match(id, body.into_inner()).await {
            Ok(Some(m)) => {
                broadcaster
                    .broadcast_event(&RealtimeEvent::MatchUpdate {
                        match_id: m.id,
                        tournament_id: None,
                        category_id: Some(m.tournament_category_id),
                        status: Some("live_update".to_string()),
                    })
                    .await;
                ApiResponse::success("Updated", Some(m))
            }
            Ok(None) => ApiResponse::not_found("Match not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_analytics(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match_analytics(id).await {
            Ok(Some(a)) => ApiResponse::success("OK", Some(a)),
            Ok(None) => ApiResponse::not_found("Analytics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_statistics(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match_statistics(id).await {
            Ok(Some(s)) => ApiResponse::success("OK", Some(s)),
            Ok(None) => ApiResponse::not_found("Statistics not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_media(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match_media(id).await {
            Ok(media) => ApiResponse::success("OK", Some(media)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn upload_video(
        use_cases: web::Data<MatchUseCasesData>,
        cloudinary: web::Data<std::sync::Arc<CloudinaryClient>>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        mut payload: Multipart,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let id = path.into_inner();
        let bytes = match extract_file_from_multipart(&mut payload, 50 * 1024 * 1024).await {
            Ok(b) => b,
            Err(r) => return r,
        };
        let public_id = format!("tournamint/matches/{}/{}", id, Uuid::new_v4());
        let result = match cloudinary.upload(&bytes, "video", &public_id).await {
            Ok(r) => r,
            Err(e) => return e.error_response(),
        };
        match use_cases
            .upload_match_media(id, user_id, "video", &result.secure_url)
            .await
        {
            Ok(media) => ApiResponse::created("Created", media),
            Err(e) => e.error_response(),
        }
    }

    pub async fn upload_photo(
        use_cases: web::Data<MatchUseCasesData>,
        cloudinary: web::Data<std::sync::Arc<CloudinaryClient>>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        mut payload: Multipart,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let id = path.into_inner();
        let bytes = match extract_file_from_multipart(&mut payload, 10 * 1024 * 1024).await {
            Ok(b) => b,
            Err(r) => return r,
        };
        let public_id = format!("tournamint/matches/{}/{}", id, Uuid::new_v4());
        let result = match cloudinary.upload(&bytes, "image", &public_id).await {
            Ok(r) => r,
            Err(e) => return e.error_response(),
        };
        match use_cases
            .upload_match_media(id, user_id, "photo", &result.secure_url)
            .await
        {
            Ok(media) => ApiResponse::created("Created", media),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_comments(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_match_comments(id).await {
            Ok(comments) => ApiResponse::success("OK", Some(comments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn add_comment(
        use_cases: web::Data<MatchUseCasesData>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        body: web::Json<AddMatchCommentRequest>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let id = path.into_inner();
        match use_cases.add_match_comment(id, user_id, &body.comment).await {
            Ok(comment) => ApiResponse::created("Created", comment),
            Err(e) => e.error_response(),
        }
    }

    pub async fn subscribe(
        use_cases: web::Data<MatchUseCasesData>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let id = path.into_inner();
        match use_cases.subscribe_to_match(id, user_id).await {
            Ok(sub) => ApiResponse::success("Subscribed", Some(sub)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn unsubscribe(
        use_cases: web::Data<MatchUseCasesData>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let id = path.into_inner();
        match use_cases.unsubscribe_from_match(id, user_id).await {
            Ok(()) => ApiResponse::success("Unsubscribed", Some(serde_json::json!({}))),
            Err(e) => e.error_response(),
        }
    }

    pub async fn bulk_update(
        use_cases: web::Data<MatchUseCasesData>,
        body: web::Json<BulkMatchUpdateBody>,
    ) -> HttpResponse {
        match use_cases
            .bulk_update_matches(body.match_ids.clone(), body.updates.clone())
            .await
        {
            Ok(matches) => ApiResponse::success("Updated", Some(matches)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn bulk_cancel(
        use_cases: web::Data<MatchUseCasesData>,
        body: web::Json<BulkCancelMatchesRequest>,
    ) -> HttpResponse {
        match use_cases
            .bulk_cancel_matches(body.match_ids.clone(), &body.reason)
            .await
        {
            Ok(matches) => ApiResponse::success("Cancelled", Some(matches)),
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

    pub async fn delete(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_match_result(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Match result not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_match(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<MatchIdPath>,
    ) -> HttpResponse {
        match use_cases.get_match_results(path.match_id).await {
            Ok(results) => ApiResponse::success("OK", Some(results)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_summary(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<MatchIdPath>,
    ) -> HttpResponse {
        match use_cases.get_match_score_summary(path.match_id).await {
            Ok(Some(summary)) => ApiResponse::success("OK", Some(summary)),
            Ok(None) => ApiResponse::not_found("Summary not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_count(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<MatchIdPath>,
    ) -> HttpResponse {
        match use_cases.count_match_results(path.match_id).await {
            Ok(count) => ApiResponse::success("OK", Some(serde_json::json!({ "count": count }))),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete_all(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<MatchIdPath>,
    ) -> HttpResponse {
        match use_cases.delete_all_match_results(path.match_id).await {
            Ok(count) => ApiResponse::success("Deleted", Some(serde_json::json!({ "deleted": count }))),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_set(
        use_cases: web::Data<MatchUseCasesData>,
        path: web::Path<MatchSetPath>,
    ) -> HttpResponse {
        match use_cases
            .get_match_results_by_set(path.match_id, path.set_number)
            .await
        {
            Ok(results) => ApiResponse::success("OK", Some(results)),
            Err(e) => e.error_response(),
        }
    }
}
