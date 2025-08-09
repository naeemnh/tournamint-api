use chrono::Utc;
use rust_decimal::Decimal;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::payment::{NewPayment, Payment, PaymentIden, PaymentStatus, PaymentStatusUpdate, PaymentSummary};

pub struct PaymentRepository;

impl PaymentRepository {
    pub async fn create(
        tx: &mut PgConnection,
        new_payment: NewPayment,
    ) -> Result<Payment, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(PaymentIden::Table)
            .columns([
                PaymentIden::UserId,
                PaymentIden::TournamentId,
                PaymentIden::Amount,
                PaymentIden::Currency,
                PaymentIden::PaymentMethod,
                PaymentIden::TransactionId,
                PaymentIden::PaymentProvider,
                PaymentIden::Metadata,
            ])
            .values([
                new_payment.user_id.into(),
                new_payment.tournament_id.into(),
                new_payment.amount.into(),
                new_payment.currency.into(),
                format!("{:?}", new_payment.payment_method).to_lowercase().into(),
                new_payment.transaction_id.into(),
                new_payment.payment_provider.into(),
                new_payment.metadata.into(),
            ])
            .unwrap()
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn find_by_id(
        tx: &mut PgConnection,
        payment_id: Uuid,
    ) -> Result<Option<Payment>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                PaymentIden::Id,
                PaymentIden::UserId,
                PaymentIden::TournamentId,
                PaymentIden::Amount,
                PaymentIden::Currency,
                PaymentIden::PaymentMethod,
                PaymentIden::Status,
                PaymentIden::TransactionId,
                PaymentIden::PaymentProvider,
                PaymentIden::ProviderPaymentId,
                PaymentIden::FailureReason,
                PaymentIden::RefundedAmount,
                PaymentIden::Metadata,
                PaymentIden::CreatedAt,
                PaymentIden::UpdatedAt,
                PaymentIden::ProcessedAt,
            ])
            .from(PaymentIden::Table)
            .and_where(Expr::col(PaymentIden::Id).eq(payment_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn find_by_user_id(
        tx: &mut PgConnection,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Payment>, sqlx::Error> {
        let mut query_builder = Query::select();
        query_builder
            .columns([
                PaymentIden::Id,
                PaymentIden::UserId,
                PaymentIden::TournamentId,
                PaymentIden::Amount,
                PaymentIden::Currency,
                PaymentIden::PaymentMethod,
                PaymentIden::Status,
                PaymentIden::TransactionId,
                PaymentIden::PaymentProvider,
                PaymentIden::ProviderPaymentId,
                PaymentIden::FailureReason,
                PaymentIden::RefundedAmount,
                PaymentIden::Metadata,
                PaymentIden::CreatedAt,
                PaymentIden::UpdatedAt,
                PaymentIden::ProcessedAt,
            ])
            .from(PaymentIden::Table)
            .and_where(Expr::col(PaymentIden::UserId).eq(user_id))
            .order_by(PaymentIden::CreatedAt, sea_query::Order::Desc);

        if let Some(limit) = limit {
            query_builder.limit(limit as u64);
        }

        if let Some(offset) = offset {
            query_builder.offset(offset as u64);
        }

        let (sql, values) = query_builder.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn find_by_tournament_id(
        tx: &mut PgConnection,
        tournament_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Payment>, sqlx::Error> {
        let mut query_builder = Query::select();
        query_builder
            .columns([
                PaymentIden::Id,
                PaymentIden::UserId,
                PaymentIden::TournamentId,
                PaymentIden::Amount,
                PaymentIden::Currency,
                PaymentIden::PaymentMethod,
                PaymentIden::Status,
                PaymentIden::TransactionId,
                PaymentIden::PaymentProvider,
                PaymentIden::ProviderPaymentId,
                PaymentIden::FailureReason,
                PaymentIden::RefundedAmount,
                PaymentIden::Metadata,
                PaymentIden::CreatedAt,
                PaymentIden::UpdatedAt,
                PaymentIden::ProcessedAt,
            ])
            .from(PaymentIden::Table)
            .and_where(Expr::col(PaymentIden::TournamentId).eq(tournament_id))
            .order_by(PaymentIden::CreatedAt, sea_query::Order::Desc);

        if let Some(limit) = limit {
            query_builder.limit(limit as u64);
        }

        if let Some(offset) = offset {
            query_builder.offset(offset as u64);
        }

        let (sql, values) = query_builder.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn update_status(
        tx: &mut PgConnection,
        payment_id: Uuid,
        status_update: PaymentStatusUpdate,
    ) -> Result<Option<Payment>, sqlx::Error> {
        let mut query_builder = Query::update();
        query_builder
            .table(PaymentIden::Table)
            .value(PaymentIden::Status, format!("{:?}", status_update.status).to_lowercase())
            .value(PaymentIden::UpdatedAt, Utc::now());

        if let Some(provider_payment_id) = status_update.provider_payment_id {
            query_builder.value(PaymentIden::ProviderPaymentId, provider_payment_id);
        }

        if let Some(failure_reason) = status_update.failure_reason {
            query_builder.value(PaymentIden::FailureReason, failure_reason);
        }

        if let Some(transaction_id) = status_update.transaction_id {
            query_builder.value(PaymentIden::TransactionId, transaction_id);
        }

        if let Some(processed_at) = status_update.processed_at {
            query_builder.value(PaymentIden::ProcessedAt, processed_at);
        }

        query_builder
            .and_where(Expr::col(PaymentIden::Id).eq(payment_id))
            .returning_all();

        let (sql, values) = query_builder.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn refund(
        tx: &mut PgConnection,
        payment_id: Uuid,
        refund_amount: Decimal,
    ) -> Result<Option<Payment>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(PaymentIden::Table)
            .values([
                (PaymentIden::RefundedAmount, refund_amount.into()),
                (PaymentIden::Status, format!("{:?}", PaymentStatus::Refunded).to_lowercase().into()),
                (PaymentIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(PaymentIden::Id).eq(payment_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn partial_refund(
        tx: &mut PgConnection,
        payment_id: Uuid,
        additional_refund_amount: Decimal,
    ) -> Result<Option<Payment>, sqlx::Error> {
        let sql = r#"
            UPDATE payments 
            SET refunded_amount = refunded_amount + $2,
                status = CASE 
                    WHEN refunded_amount + $2 >= amount THEN 'refunded'::payment_status
                    ELSE 'partial_refund'::payment_status
                END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
        "#;

        sqlx::query_as::<_, Payment>(sql)
            .bind(payment_id)
            .bind(additional_refund_amount)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn get_summary_by_tournament(
        tx: &mut PgConnection,
        tournament_id: Uuid,
    ) -> Result<PaymentSummary, sqlx::Error> {
        let sql = r#"
            SELECT 
                COUNT(*) as total_payments,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(CASE WHEN status = 'completed' THEN 1 END) as successful_payments,
                COALESCE(SUM(CASE WHEN status = 'completed' THEN amount ELSE 0 END), 0) as successful_amount,
                COUNT(CASE WHEN status = 'failed' THEN 1 END) as failed_payments,
                COUNT(CASE WHEN status = 'pending' THEN 1 END) as pending_payments
            FROM payments 
            WHERE tournament_id = $1
        "#;

        sqlx::query_as::<_, PaymentSummary>(sql)
            .bind(tournament_id)
            .fetch_one(&mut *tx)
            .await
    }

    pub async fn get_summary_by_user(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<PaymentSummary, sqlx::Error> {
        let sql = r#"
            SELECT 
                COUNT(*) as total_payments,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(CASE WHEN status = 'completed' THEN 1 END) as successful_payments,
                COALESCE(SUM(CASE WHEN status = 'completed' THEN amount ELSE 0 END), 0) as successful_amount,
                COUNT(CASE WHEN status = 'failed' THEN 1 END) as failed_payments,
                COUNT(CASE WHEN status = 'pending' THEN 1 END) as pending_payments
            FROM payments 
            WHERE user_id = $1
        "#;

        sqlx::query_as::<_, PaymentSummary>(sql)
            .bind(user_id)
            .fetch_one(&mut *tx)
            .await
    }
}