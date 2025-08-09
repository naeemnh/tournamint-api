use std::fmt::Write;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartialRefund,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_method", rename_all = "snake_case")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    PayPal,
    BankTransfer,
    Stripe,
    Other,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub tournament_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub transaction_id: Option<String>,
    pub payment_provider: Option<String>,
    pub provider_payment_id: Option<String>,
    pub failure_reason: Option<String>,
    pub refunded_amount: Option<Decimal>,
    pub metadata: Option<serde_json::Value>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPayment {
    pub user_id: Uuid,
    pub tournament_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub transaction_id: Option<String>,
    pub payment_provider: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessPaymentRequest {
    pub tournament_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethod,
    pub payment_provider: String,
    pub provider_payment_id: String,
    pub transaction_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundRequest {
    pub amount: Option<Decimal>, // If None, full refund
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStatusUpdate {
    pub status: PaymentStatus,
    pub provider_payment_id: Option<String>,
    pub failure_reason: Option<String>,
    pub transaction_id: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PaymentSummary {
    pub total_payments: i64,
    pub total_amount: Decimal,
    pub successful_payments: i64,
    pub successful_amount: Decimal,
    pub failed_payments: i64,
    pub pending_payments: i64,
}

pub enum PaymentIden {
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