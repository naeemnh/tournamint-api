use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartialRefund,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    PayPal,
    BankTransfer,
    Stripe,
    Other,
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
    pub metadata: Option<Value>,
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
    pub metadata: Option<Value>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSummary {
    pub total_payments: i64,
    pub total_amount: Decimal,
    pub successful_payments: i64,
    pub successful_amount: Decimal,
    pub failed_payments: i64,
    pub pending_payments: i64,
}
