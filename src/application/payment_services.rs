use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::payment::{
    NewPayment, Payment, PaymentRepository, PaymentStatus, PaymentSummary,
};
use crate::shared::AppError;

/// Payment domain use cases
pub struct PaymentServices<R>
where
    R: PaymentRepository,
{
    payment_repo: Arc<R>,
}

impl<R> PaymentServices<R>
where
    R: PaymentRepository,
{
    pub fn new(payment_repo: Arc<R>) -> Self {
        Self { payment_repo }
    }

    pub async fn process_payment(&self, data: NewPayment) -> Result<Payment, AppError> {
        self.payment_repo.create(data).await
    }

    pub async fn get_payment(&self, payment_id: Uuid) -> Result<Option<Payment>, AppError> {
        self.payment_repo.find_by_id(payment_id).await
    }

    pub async fn get_user_payments(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Payment>, AppError> {
        self.payment_repo
            .find_by_user_id(user_id, limit, offset)
            .await
    }

    pub async fn get_tournament_payments(
        &self,
        tournament_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Payment>, AppError> {
        self.payment_repo
            .find_by_tournament_id(tournament_id, limit, offset)
            .await
    }

    pub async fn update_payment_status(
        &self,
        payment_id: Uuid,
        status: PaymentStatus,
    ) -> Result<Option<Payment>, AppError> {
        self.payment_repo.update_status(payment_id, status).await
    }

    pub async fn refund_payment(
        &self,
        payment_id: Uuid,
        amount: Option<Decimal>,
    ) -> Result<Option<Payment>, AppError> {
        // Get the payment to determine full amount if needed
        let payment = self.payment_repo.find_by_id(payment_id).await?;

        match payment {
            Some(p) => {
                let refund_amount = amount.unwrap_or(p.amount);
                if refund_amount == p.amount {
                    self.payment_repo.refund(payment_id, refund_amount).await
                } else {
                    self.payment_repo
                        .partial_refund(payment_id, refund_amount)
                        .await
                }
            }
            None => Ok(None),
        }
    }

    pub async fn get_tournament_payment_summary(
        &self,
        tournament_id: Uuid,
    ) -> Result<PaymentSummary, AppError> {
        self.payment_repo
            .get_summary_by_tournament(tournament_id)
            .await
    }

    pub async fn get_user_payment_summary(
        &self,
        user_id: Uuid,
    ) -> Result<PaymentSummary, AppError> {
        self.payment_repo.get_summary_by_user(user_id).await
    }
}
