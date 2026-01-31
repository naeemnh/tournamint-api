use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::value_objects::{PaymentMethod, PaymentStatus};

/// Payment entity representing a financial transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub metadata: Option<Value>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds_option")]
    pub processed_at: Option<DateTime<Utc>>,
}
