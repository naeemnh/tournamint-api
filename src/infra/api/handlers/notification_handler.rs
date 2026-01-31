use actix_web::{web, HttpRequest, HttpResponse, ResponseError};

use crate::application::NotificationUseCases;
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::db::PgNotificationRepository;
use crate::shared::ApiResponse;

type NotificationUseCasesData = std::sync::Arc<NotificationUseCases<PgNotificationRepository>>;

pub struct NotificationHandler;

impl NotificationHandler {
    pub async fn index(
        use_cases: web::Data<NotificationUseCasesData>,
        req: HttpRequest,
        query: web::Query<NotificationQuery>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0).max(0);
        match use_cases.get_notifications(user_id, limit, offset).await {
            Ok(notifications) => ApiResponse::success("OK", Some(notifications)),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct NotificationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
