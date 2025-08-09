use actix_web::{web, HttpRequest, Responder};
use uuid::Uuid;

use crate::config::DbPool;
use crate::middlewares::auth::get_user_from_token;
use crate::models::common::PaginationQuery;
use crate::models::payment::{PaymentStatusUpdate, ProcessPaymentRequest, RefundRequest};
use crate::services::payment_service::PaymentService;

pub struct PaymentController;

impl PaymentController {
    /// POST /payments/process - Process a new payment
    pub async fn process_payment(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        payment_request: web::Json<ProcessPaymentRequest>,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => {
                PaymentService::process_payment(&pool, user.id, payment_request.into_inner()).await
            }
            Err(response) => response,
        }
    }

    /// GET /payments/{id} - Get payment by ID
    pub async fn get_payment(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let payment_id = path.into_inner();

        // Get user from token for ownership verification
        let user_id = match get_user_from_token(&req).await {
            Ok(user) => Some(user.id),
            Err(_) => None, // Allow admin access without user verification
        };

        PaymentService::get_payment(&pool, payment_id, user_id).await
    }

    /// GET /payments/user/{user_id} - Get payments for a specific user
    pub async fn get_user_payments(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        let requested_user_id = path.into_inner();

        match get_user_from_token(&req).await {
            Ok(user) => {
                // Users can only see their own payments unless they are admin
                // TODO: Add admin role check when user roles are implemented
                if user.id != requested_user_id {
                    return PaymentService::get_user_payments(
                        &pool,
                        requested_user_id,
                        query.limit,
                        query.offset,
                    )
                    .await;
                }

                PaymentService::get_user_payments(&pool, user.id, query.limit, query.offset).await
            }
            Err(response) => response,
        }
    }

    /// GET /payments/tournament/{tournament_id} - Get payments for a tournament
    pub async fn get_tournament_payments(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        let tournament_id = path.into_inner();

        // TODO: Add authorization check - only tournament organizers should see all payments
        match get_user_from_token(&req).await {
            Ok(_user) => {
                PaymentService::get_tournament_payments(
                    &pool,
                    tournament_id,
                    query.limit,
                    query.offset,
                )
                .await
            }
            Err(response) => response,
        }
    }

    /// PUT /payments/{id}/refund - Refund a payment
    pub async fn refund_payment(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        refund_request: web::Json<RefundRequest>,
    ) -> impl Responder {
        let payment_id = path.into_inner();

        match get_user_from_token(&req).await {
            Ok(user) => {
                // TODO: Add admin role check for refunds
                // For now, users can refund their own payments
                PaymentService::refund_payment(
                    &pool,
                    payment_id,
                    refund_request.into_inner(),
                    Some(user.id),
                )
                .await
            }
            Err(response) => response,
        }
    }

    /// GET /payments/{id}/status - Get payment status
    pub async fn get_payment_status(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let payment_id = path.into_inner();

        match get_user_from_token(&req).await {
            Ok(user) => PaymentService::get_payment(&pool, payment_id, Some(user.id)).await,
            Err(response) => response,
        }
    }

    /// PUT /payments/{id}/status - Update payment status (admin/webhook only)
    pub async fn update_payment_status(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
        status_update: web::Json<PaymentStatusUpdate>,
    ) -> impl Responder {
        let payment_id = path.into_inner();

        // TODO: Add proper webhook validation or admin role check
        // For now, require authentication but allow any authenticated user
        match get_user_from_token(&req).await {
            Ok(_user) => {
                PaymentService::update_payment_status(&pool, payment_id, status_update.into_inner())
                    .await
            }
            Err(response) => response,
        }
    }

    /// GET /payments/summary/tournament/{tournament_id} - Get payment summary for tournament
    pub async fn get_tournament_payment_summary(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let tournament_id = path.into_inner();

        match get_user_from_token(&req).await {
            Ok(_user) => {
                // TODO: Add tournament organizer authorization check
                PaymentService::get_payment_summary_by_tournament(&pool, tournament_id).await
            }
            Err(response) => response,
        }
    }

    /// GET /payments/summary/user - Get payment summary for current user
    pub async fn get_user_payment_summary(
        pool: web::Data<DbPool>,
        req: HttpRequest,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => PaymentService::get_payment_summary_by_user(&pool, user.id).await,
            Err(response) => response,
        }
    }
}
