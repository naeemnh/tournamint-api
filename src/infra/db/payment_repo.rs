use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_query::Iden;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::payment::{
    NewPayment, Payment, PaymentMethod, PaymentRepository, PaymentStatus, PaymentSummary,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden ====================

enum PaymentIden {
    Table,
    Id,
    UserId,
    TournamentId,
    Amount,
    Currency,
    PaymentMethod,
    Status,
    TransactionId,
    PaymentProvider,
    ProviderPaymentId,
    FailureReason,
    RefundedAmount,
    Metadata,
    CreatedAt,
    UpdatedAt,
    ProcessedAt,
}

impl Iden for PaymentIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                PaymentIden::Table => "payments",
                PaymentIden::Id => "id",
                PaymentIden::UserId => "user_id",
                PaymentIden::TournamentId => "tournament_id",
                PaymentIden::Amount => "amount",
                PaymentIden::Currency => "currency",
                PaymentIden::PaymentMethod => "payment_method",
                PaymentIden::Status => "status",
                PaymentIden::TransactionId => "transaction_id",
                PaymentIden::PaymentProvider => "payment_provider",
                PaymentIden::ProviderPaymentId => "provider_payment_id",
                PaymentIden::FailureReason => "failure_reason",
                PaymentIden::RefundedAmount => "refunded_amount",
                PaymentIden::Metadata => "metadata",
                PaymentIden::CreatedAt => "created_at",
                PaymentIden::UpdatedAt => "updated_at",
                PaymentIden::ProcessedAt => "processed_at",
            }
        )
        .unwrap()
    }
}

// ==================== Enum mapping ====================

fn payment_status_to_db(s: &PaymentStatus) -> &'static str {
    match s {
        PaymentStatus::Pending => "pending",
        PaymentStatus::Processing => "processing",
        PaymentStatus::Completed => "completed",
        PaymentStatus::Failed => "failed",
        PaymentStatus::Cancelled => "cancelled",
        PaymentStatus::Refunded => "refunded",
        PaymentStatus::PartialRefund => "partial_refund",
    }
}

fn payment_status_from_db(s: &str) -> Option<PaymentStatus> {
    match s {
        "pending" => Some(PaymentStatus::Pending),
        "processing" => Some(PaymentStatus::Processing),
        "completed" => Some(PaymentStatus::Completed),
        "failed" => Some(PaymentStatus::Failed),
        "cancelled" => Some(PaymentStatus::Cancelled),
        "refunded" => Some(PaymentStatus::Refunded),
        "partial_refund" => Some(PaymentStatus::PartialRefund),
        _ => None,
    }
}

fn payment_method_to_db(m: &PaymentMethod) -> &'static str {
    match m {
        PaymentMethod::CreditCard => "credit_card",
        PaymentMethod::DebitCard => "debit_card",
        PaymentMethod::PayPal => "paypal",
        PaymentMethod::BankTransfer => "bank_transfer",
        PaymentMethod::Stripe => "stripe",
        PaymentMethod::Other => "other",
    }
}

fn payment_method_from_db(s: &str) -> Option<PaymentMethod> {
    match s {
        "credit_card" => Some(PaymentMethod::CreditCard),
        "debit_card" => Some(PaymentMethod::DebitCard),
        "paypal" => Some(PaymentMethod::PayPal),
        "bank_transfer" => Some(PaymentMethod::BankTransfer),
        "stripe" => Some(PaymentMethod::Stripe),
        "other" => Some(PaymentMethod::Other),
        _ => None,
    }
}

// ==================== Row types ====================

#[derive(Debug, FromRow)]
struct PaymentRow {
    id: Uuid,
    user_id: Uuid,
    tournament_id: Uuid,
    amount: Decimal,
    currency: String,
    payment_method: String,
    status: String,
    transaction_id: Option<String>,
    payment_provider: Option<String>,
    provider_payment_id: Option<String>,
    failure_reason: Option<String>,
    refunded_amount: Option<Decimal>,
    metadata: Option<JsonValue>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
    processed_at: Option<chrono::DateTime<Utc>>,
}

impl From<PaymentRow> for Payment {
    fn from(row: PaymentRow) -> Self {
        Payment {
            id: row.id,
            user_id: row.user_id,
            tournament_id: row.tournament_id,
            amount: row.amount,
            currency: row.currency,
            payment_method: payment_method_from_db(&row.payment_method)
                .unwrap_or(PaymentMethod::Other),
            status: payment_status_from_db(&row.status).unwrap_or(PaymentStatus::Pending),
            transaction_id: row.transaction_id,
            payment_provider: row.payment_provider,
            provider_payment_id: row.provider_payment_id,
            failure_reason: row.failure_reason,
            refunded_amount: row.refunded_amount,
            metadata: row.metadata,
            created_at: row.created_at,
            updated_at: row.updated_at,
            processed_at: row.processed_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct PaymentSummaryRow {
    total_payments: i64,
    total_amount: Decimal,
    successful_payments: i64,
    successful_amount: Decimal,
    failed_payments: i64,
    pending_payments: i64,
}

impl From<PaymentSummaryRow> for PaymentSummary {
    fn from(row: PaymentSummaryRow) -> Self {
        PaymentSummary {
            total_payments: row.total_payments,
            total_amount: row.total_amount,
            successful_payments: row.successful_payments,
            successful_amount: row.successful_amount,
            failed_payments: row.failed_payments,
            pending_payments: row.pending_payments,
        }
    }
}

// SELECT list with enums as text for decoding into PaymentRow
const PAYMENT_SELECT: &str = "id, user_id, tournament_id, amount, currency, payment_method::text as payment_method, status::text as status, transaction_id, payment_provider, provider_payment_id, failure_reason, refunded_amount, metadata, created_at, updated_at, processed_at";

// ==================== Repository ====================

pub struct PgPaymentRepository {
    pool: DbPool,
}

impl PgPaymentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentRepository for PgPaymentRepository {
    async fn create(&self, new_payment: NewPayment) -> Result<Payment, AppError> {
        let sql = r#"
            INSERT INTO payments (user_id, tournament_id, amount, currency, payment_method, transaction_id, payment_provider, metadata)
            VALUES ($1, $2, $3, $4, $5::payment_method, $6, $7, $8)
            RETURNING id, user_id, tournament_id, amount, currency, payment_method::text as payment_method, status::text as status,
                transaction_id, payment_provider, provider_payment_id, failure_reason, refunded_amount, metadata, created_at, updated_at, processed_at
        "#;
        let row: PaymentRow = sqlx::query_as(sql)
            .bind(new_payment.user_id)
            .bind(new_payment.tournament_id)
            .bind(new_payment.amount)
            .bind(&new_payment.currency)
            .bind(payment_method_to_db(&new_payment.payment_method))
            .bind(new_payment.transaction_id)
            .bind(new_payment.payment_provider)
            .bind(new_payment.metadata)
            .fetch_one(&self.pool)
            .await?;
        Ok(Payment::from(row))
    }

    async fn find_by_id(&self, payment_id: Uuid) -> Result<Option<Payment>, AppError> {
        let sql = format!("SELECT {} FROM payments WHERE id = $1", PAYMENT_SELECT);
        let row: Option<PaymentRow> = sqlx::query_as(&sql)
            .bind(payment_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Payment::from))
    }

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Payment>, AppError> {
        let sql = format!(
            "SELECT {} FROM payments WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            PAYMENT_SELECT
        );
        let rows: Vec<PaymentRow> = sqlx::query_as(&sql)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Payment::from).collect())
    }

    async fn find_by_tournament_id(
        &self,
        tournament_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Payment>, AppError> {
        let sql = format!(
            "SELECT {} FROM payments WHERE tournament_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            PAYMENT_SELECT
        );
        let rows: Vec<PaymentRow> = sqlx::query_as(&sql)
            .bind(tournament_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Payment::from).collect())
    }

    async fn update_status(
        &self,
        payment_id: Uuid,
        status: PaymentStatus,
    ) -> Result<Option<Payment>, AppError> {
        let sql = r#"
            UPDATE payments SET status = $2::payment_status, updated_at = $3
            WHERE id = $1
            RETURNING id, user_id, tournament_id, amount, currency, payment_method::text as payment_method, status::text as status,
                transaction_id, payment_provider, provider_payment_id, failure_reason, refunded_amount, metadata, created_at, updated_at, processed_at
        "#;
        let row: Option<PaymentRow> = sqlx::query_as(sql)
            .bind(payment_id)
            .bind(payment_status_to_db(&status))
            .bind(Utc::now())
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Payment::from))
    }

    async fn refund(&self, payment_id: Uuid, amount: Decimal) -> Result<Option<Payment>, AppError> {
        let sql = r#"
            UPDATE payments SET refunded_amount = $2, status = 'refunded'::payment_status, updated_at = $3
            WHERE id = $1
            RETURNING id, user_id, tournament_id, amount, currency, payment_method::text as payment_method, status::text as status,
                transaction_id, payment_provider, provider_payment_id, failure_reason, refunded_amount, metadata, created_at, updated_at, processed_at
        "#;
        let row: Option<PaymentRow> = sqlx::query_as(sql)
            .bind(payment_id)
            .bind(amount)
            .bind(Utc::now())
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Payment::from))
    }

    async fn partial_refund(
        &self,
        payment_id: Uuid,
        amount: Decimal,
    ) -> Result<Option<Payment>, AppError> {
        let sql = r#"
            UPDATE payments 
            SET refunded_amount = COALESCE(refunded_amount, 0) + $2,
                status = CASE 
                    WHEN COALESCE(refunded_amount, 0) + $2 >= amount THEN 'refunded'::payment_status
                    ELSE 'partial_refund'::payment_status
                END,
                updated_at = $3
            WHERE id = $1
            RETURNING id, user_id, tournament_id, amount, currency, payment_method::text as payment_method, status::text as status,
                transaction_id, payment_provider, provider_payment_id, failure_reason, refunded_amount, metadata, created_at, updated_at, processed_at
        "#;
        let row: Option<PaymentRow> = sqlx::query_as(sql)
            .bind(payment_id)
            .bind(amount)
            .bind(Utc::now())
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Payment::from))
    }

    async fn get_summary_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<PaymentSummary, AppError> {
        let sql = r#"
            SELECT 
                COUNT(*)::bigint as total_payments,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(CASE WHEN status = 'completed' THEN 1 END)::bigint as successful_payments,
                COALESCE(SUM(CASE WHEN status = 'completed' THEN amount ELSE 0 END), 0) as successful_amount,
                COUNT(CASE WHEN status = 'failed' THEN 1 END)::bigint as failed_payments,
                COUNT(CASE WHEN status = 'pending' THEN 1 END)::bigint as pending_payments
            FROM payments 
            WHERE tournament_id = $1
        "#;
        let row: PaymentSummaryRow = sqlx::query_as(sql)
            .bind(tournament_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(PaymentSummary::from(row))
    }

    async fn get_summary_by_user(&self, user_id: Uuid) -> Result<PaymentSummary, AppError> {
        let sql = r#"
            SELECT 
                COUNT(*)::bigint as total_payments,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(CASE WHEN status = 'completed' THEN 1 END)::bigint as successful_payments,
                COALESCE(SUM(CASE WHEN status = 'completed' THEN amount ELSE 0 END), 0) as successful_amount,
                COUNT(CASE WHEN status = 'failed' THEN 1 END)::bigint as failed_payments,
                COUNT(CASE WHEN status = 'pending' THEN 1 END)::bigint as pending_payments
            FROM payments 
            WHERE user_id = $1
        "#;
        let row: PaymentSummaryRow = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(PaymentSummary::from(row))
    }
}
