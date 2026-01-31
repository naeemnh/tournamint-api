use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::PaymentUseCases;
use crate::domain::payment::{NewPayment, PaymentStatus};
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::db::PgPaymentRepository;
use crate::shared::ApiResponse;

type PaymentUseCasesData = std::sync::Arc<PaymentUseCases<PgPaymentRepository>>;

#[derive(Debug, Deserialize)]
pub struct PaymentIdPath {
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UserIdPath {
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TournamentIdPath {
    pub tournament_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct RefundBody {
    pub amount: Option<rust_decimal::Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentStatusBody {
    pub status: PaymentStatus,
}

pub struct PaymentHandler;

impl PaymentHandler {
    pub async fn process(
        use_cases: web::Data<PaymentUseCasesData>,
        body: web::Json<NewPayment>,
    ) -> HttpResponse {
        match use_cases.process_payment(body.into_inner()).await {
            Ok(payment) => ApiResponse::created("Created", payment),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<PaymentIdPath>,
    ) -> HttpResponse {
        match use_cases.get_payment(path.id).await {
            Ok(Some(payment)) => ApiResponse::success("OK", Some(payment)),
            Ok(None) => ApiResponse::not_found("Payment not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_user(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<UserIdPath>,
        query: web::Query<PaymentListQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0).max(0);
        match use_cases.get_user_payments(path.user_id, limit, offset).await {
            Ok(payments) => ApiResponse::success("OK", Some(payments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_tournament(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<TournamentIdPath>,
        query: web::Query<PaymentListQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0).max(0);
        match use_cases
            .get_tournament_payments(path.tournament_id, limit, offset)
            .await
        {
            Ok(payments) => ApiResponse::success("OK", Some(payments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn refund(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<PaymentIdPath>,
        body: Option<web::Json<RefundBody>>,
    ) -> HttpResponse {
        let amount = body.and_then(|b| b.amount);
        match use_cases.refund_payment(path.id, amount).await {
            Ok(Some(payment)) => ApiResponse::success("Refunded", Some(payment)),
            Ok(None) => ApiResponse::not_found("Payment not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_status(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<PaymentIdPath>,
    ) -> HttpResponse {
        match use_cases.get_payment(path.id).await {
            Ok(Some(payment)) => ApiResponse::success("OK", Some(serde_json::json!({ "status": payment.status }))),
            Ok(None) => ApiResponse::not_found("Payment not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_status(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<PaymentIdPath>,
        body: web::Json<PaymentStatusBody>,
    ) -> HttpResponse {
        match use_cases.update_payment_status(path.id, body.status.clone()).await {
            Ok(Some(payment)) => ApiResponse::success("Updated", Some(payment)),
            Ok(None) => ApiResponse::not_found("Payment not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_tournament_summary(
        use_cases: web::Data<PaymentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases.get_tournament_payment_summary(path.tournament_id).await {
            Ok(summary) => ApiResponse::success("OK", Some(summary)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_user_summary(
        use_cases: web::Data<PaymentUseCasesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.get_user_payment_summary(user_id).await {
            Ok(summary) => ApiResponse::success("OK", Some(summary)),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PaymentListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
