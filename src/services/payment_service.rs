use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use rust_decimal::Decimal;
use uuid::Uuid;
use chrono::Utc;

use crate::config::DbPool;
use crate::formatters;
use crate::models::payment::{
    NewPayment, Payment, PaymentStatus, PaymentStatusUpdate, ProcessPaymentRequest, RefundRequest,
};
use crate::repositories::payment_repository::PaymentRepository;
use crate::utils::db::with_transaction;

pub struct PaymentService;

impl PaymentService {
    pub async fn process_payment(
        pool: &DbPool,
        user_id: Uuid,
        payment_request: ProcessPaymentRequest,
    ) -> HttpResponse {
        let new_payment = NewPayment {
            user_id,
            tournament_id: payment_request.tournament_id,
            amount: payment_request.amount,
            currency: payment_request.currency,
            payment_method: payment_request.payment_method,
            transaction_id: payment_request.transaction_id,
            payment_provider: Some(payment_request.payment_provider),
            metadata: payment_request.metadata,
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                // Create payment record
                let payment = PaymentRepository::create(tx, new_payment).await?;
                
                // Update status to processing with provider ID
                let status_update = PaymentStatusUpdate {
                    status: PaymentStatus::Processing,
                    provider_payment_id: Some(payment_request.provider_payment_id),
                    failure_reason: None,
                    transaction_id: None,
                    processed_at: Some(Utc::now()),
                };
                
                PaymentRepository::update_status(tx, payment.id, status_update).await
            })
        })
        .await
        {
            Ok(Some(payment)) => {
                formatters::success_response(StatusCode::CREATED, payment, "PAYMENT_PROCESSING")
            }
            Ok(None) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Payment creation failed",
                "PAYMENT_CREATION_ERROR",
            ),
            Err(e) => {
                let error = e.to_string();
                let error_message = if error.contains("foreign key constraint") {
                    "Invalid tournament or user ID"
                } else {
                    "Payment processing failed"
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "PAYMENT_PROCESSING_ERROR",
                )
            }
        }
    }

    pub async fn get_payment(
        pool: &DbPool,
        payment_id: Uuid,
        user_id: Option<Uuid>,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { PaymentRepository::find_by_id(tx, payment_id).await })
        })
        .await
        {
            Ok(Some(payment)) => {
                // If user_id is provided, verify ownership
                if let Some(user_id) = user_id {
                    if payment.user_id != user_id {
                        return formatters::error_response(
                            StatusCode::FORBIDDEN,
                            "Access denied",
                            "PAYMENT_ACCESS_DENIED",
                        );
                    }
                }
                formatters::success_response(StatusCode::OK, payment, "PAYMENT_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Payment not found",
                "PAYMENT_NOT_FOUND",
            ),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_user_payments(
        pool: &DbPool,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let limit = limit.unwrap_or(50).min(100); // Cap at 100
        let offset = offset.unwrap_or(0);

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                PaymentRepository::find_by_user_id(tx, user_id, Some(limit), Some(offset)).await
            })
        })
        .await
        {
            Ok(payments) => {
                formatters::success_response(StatusCode::OK, payments, "PAYMENTS_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_tournament_payments(
        pool: &DbPool,
        tournament_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let limit = limit.unwrap_or(50).min(100); // Cap at 100
        let offset = offset.unwrap_or(0);

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                PaymentRepository::find_by_tournament_id(tx, tournament_id, Some(limit), Some(offset))
                    .await
            })
        })
        .await
        {
            Ok(payments) => {
                formatters::success_response(StatusCode::OK, payments, "PAYMENTS_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn refund_payment(
        pool: &DbPool,
        payment_id: Uuid,
        refund_request: RefundRequest,
        requester_user_id: Option<Uuid>,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                // First, get the payment to check ownership and current state
                let payment = PaymentRepository::find_by_id(tx, payment_id).await?;
                
                match payment {
                    Some(payment) => {
                        // Verify ownership if user_id is provided
                        if let Some(user_id) = requester_user_id {
                            if payment.user_id != user_id {
                                return Err(sqlx::Error::RowNotFound);
                            }
                        }

                        // Check if payment can be refunded
                        if payment.status != PaymentStatus::Completed {
                            return Err(sqlx::Error::RowNotFound);
                        }

                        let refund_amount = refund_request.amount.unwrap_or(payment.amount);
                        let current_refunded = payment.refunded_amount.unwrap_or(Decimal::ZERO);
                        let available_for_refund = payment.amount - current_refunded;

                        if refund_amount > available_for_refund {
                            return Err(sqlx::Error::RowNotFound);
                        }

                        // Process refund
                        if refund_amount == available_for_refund {
                            // Full refund (or completing partial refunds)
                            PaymentRepository::refund(tx, payment_id, current_refunded + refund_amount).await
                        } else {
                            // Partial refund
                            PaymentRepository::partial_refund(tx, payment_id, refund_amount).await
                        }
                    }
                    None => Err(sqlx::Error::RowNotFound),
                }
            })
        })
        .await
        {
            Ok(Some(payment)) => {
                formatters::success_response(StatusCode::OK, payment, "PAYMENT_REFUNDED")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Payment not found or cannot be refunded",
                "REFUND_FAILED",
            ),
            Err(sqlx::Error::RowNotFound) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Payment not found or cannot be refunded",
                "REFUND_FAILED",
            ),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn update_payment_status(
        pool: &DbPool,
        payment_id: Uuid,
        status_update: PaymentStatusUpdate,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                PaymentRepository::update_status(tx, payment_id, status_update).await
            })
        })
        .await
        {
            Ok(Some(payment)) => {
                formatters::success_response(StatusCode::OK, payment, "PAYMENT_STATUS_UPDATED")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Payment not found",
                "PAYMENT_NOT_FOUND",
            ),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_payment_summary_by_tournament(
        pool: &DbPool,
        tournament_id: Uuid,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                PaymentRepository::get_summary_by_tournament(tx, tournament_id).await
            })
        })
        .await
        {
            Ok(summary) => {
                formatters::success_response(StatusCode::OK, summary, "PAYMENT_SUMMARY_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn get_payment_summary_by_user(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { PaymentRepository::get_summary_by_user(tx, user_id).await })
        })
        .await
        {
            Ok(summary) => {
                formatters::success_response(StatusCode::OK, summary, "PAYMENT_SUMMARY_FOUND")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }
}