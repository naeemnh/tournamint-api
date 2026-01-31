use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;

use super::entity::Payment;
use super::value_objects::{NewPayment, PaymentStatus, PaymentSummary};
use crate::shared::AppError;

/// Repository trait for Payment entity operations
#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn create(&self, new_payment: NewPayment) -> Result<Payment, AppError>;
    async fn find_by_id(&self, payment_id: Uuid) -> Result<Option<Payment>, AppError>;
    async fn find_by_user_id(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Payment>, AppError>;
    async fn find_by_tournament_id(&self, tournament_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Payment>, AppError>;
    async fn update_status(&self, payment_id: Uuid, status: PaymentStatus) -> Result<Option<Payment>, AppError>;
    async fn refund(&self, payment_id: Uuid, amount: Decimal) -> Result<Option<Payment>, AppError>;
    async fn partial_refund(&self, payment_id: Uuid, amount: Decimal) -> Result<Option<Payment>, AppError>;
    async fn get_summary_by_tournament(&self, tournament_id: Uuid) -> Result<PaymentSummary, AppError>;
    async fn get_summary_by_user(&self, user_id: Uuid) -> Result<PaymentSummary, AppError>;
}
