use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::tournament::{
    EditableTournamentRegistration, NewTournamentRegistration, PaymentStatus, RegistrationStatus,
    RegistrationWithDetails, TournamentRegistration, TournamentRegistrationRepository,
};
use crate::shared::AppError;

use super::pool::DbPool;
use super::tournament_repo::{
    payment_status_to_string, registration_status_to_string, PaymentStatusDb, RegistrationStatusDb,
};

// ==================== Sea-Query Iden ====================

pub enum TournamentRegistrationIden {
    Table,
    Id,
    TournamentCategoryId,
    TeamId,
    PlayerId,
    PartnerPlayerId,
    RegistrationStatus,
    PaymentStatus,
    RegistrationDate,
    ApprovalDate,
    PaymentDate,
    PaymentAmount,
    PaymentReference,
    Notes,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentRegistrationIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentRegistrationIden::Table => "tournament_registrations",
                TournamentRegistrationIden::Id => "id",
                TournamentRegistrationIden::TournamentCategoryId => "tournament_category_id",
                TournamentRegistrationIden::TeamId => "team_id",
                TournamentRegistrationIden::PlayerId => "player_id",
                TournamentRegistrationIden::PartnerPlayerId => "partner_player_id",
                TournamentRegistrationIden::RegistrationStatus => "registration_status",
                TournamentRegistrationIden::PaymentStatus => "payment_status",
                TournamentRegistrationIden::RegistrationDate => "registration_date",
                TournamentRegistrationIden::ApprovalDate => "approval_date",
                TournamentRegistrationIden::PaymentDate => "payment_date",
                TournamentRegistrationIden::PaymentAmount => "payment_amount",
                TournamentRegistrationIden::PaymentReference => "payment_reference",
                TournamentRegistrationIden::Notes => "notes",
                TournamentRegistrationIden::Metadata => "metadata",
                TournamentRegistrationIden::CreatedAt => "created_at",
                TournamentRegistrationIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TournamentRegistrationRow {
    id: Uuid,
    tournament_category_id: Uuid,
    team_id: Option<Uuid>,
    player_id: Option<Uuid>,
    partner_player_id: Option<Uuid>,
    registration_status: RegistrationStatusDb,
    payment_status: PaymentStatusDb,
    registration_date: chrono::DateTime<Utc>,
    approval_date: Option<chrono::DateTime<Utc>>,
    payment_date: Option<chrono::DateTime<Utc>>,
    payment_amount: Option<Decimal>,
    payment_reference: Option<String>,
    notes: Option<String>,
    metadata: Option<JsonValue>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<TournamentRegistrationRow> for TournamentRegistration {
    fn from(row: TournamentRegistrationRow) -> Self {
        TournamentRegistration {
            id: row.id,
            tournament_category_id: row.tournament_category_id,
            team_id: row.team_id,
            player_id: row.player_id,
            partner_player_id: row.partner_player_id,
            registration_status: row.registration_status.into(),
            payment_status: row.payment_status.into(),
            registration_date: row.registration_date,
            approval_date: row.approval_date,
            payment_date: row.payment_date,
            payment_amount: row.payment_amount,
            payment_reference: row.payment_reference,
            notes: row.notes,
            metadata: row.metadata,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct RegistrationWithDetailsRow {
    id: Uuid,
    tournament_category_id: Uuid,
    tournament_name: String,
    category_name: String,
    team_name: Option<String>,
    player_name: Option<String>,
    partner_name: Option<String>,
    registration_status: RegistrationStatusDb,
    payment_status: PaymentStatusDb,
    registration_date: chrono::DateTime<Utc>,
}

impl From<RegistrationWithDetailsRow> for RegistrationWithDetails {
    fn from(row: RegistrationWithDetailsRow) -> Self {
        RegistrationWithDetails {
            id: row.id,
            tournament_category_id: row.tournament_category_id,
            tournament_name: row.tournament_name,
            category_name: row.category_name,
            team_name: row.team_name,
            player_name: row.player_name,
            partner_name: row.partner_name,
            registration_status: row.registration_status.into(),
            payment_status: row.payment_status.into(),
            registration_date: row.registration_date,
        }
    }
}

// ==================== Repository ====================

pub struct PgTournamentRegistrationRepository {
    pool: DbPool,
}

impl PgTournamentRegistrationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TournamentRegistrationRepository for PgTournamentRegistrationRepository {
    async fn create(
        &self,
        new_registration: NewTournamentRegistration,
    ) -> Result<TournamentRegistration, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TournamentRegistrationIden::Table)
            .columns([
                TournamentRegistrationIden::TournamentCategoryId,
                TournamentRegistrationIden::TeamId,
                TournamentRegistrationIden::PlayerId,
                TournamentRegistrationIden::PartnerPlayerId,
                TournamentRegistrationIden::Notes,
                TournamentRegistrationIden::Metadata,
            ])
            .values_panic([
                new_registration.tournament_category_id.into(),
                new_registration.team_id.into(),
                new_registration.player_id.into(),
                new_registration.partner_player_id.into(),
                new_registration.notes.into(),
                new_registration.metadata.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TournamentRegistrationRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(TournamentRegistration::from(row))
    }

    async fn get_by_id(
        &self,
        registration_id: Uuid,
    ) -> Result<Option<TournamentRegistration>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentRegistrationIden::Id,
                TournamentRegistrationIden::TournamentCategoryId,
                TournamentRegistrationIden::TeamId,
                TournamentRegistrationIden::PlayerId,
                TournamentRegistrationIden::PartnerPlayerId,
                TournamentRegistrationIden::RegistrationStatus,
                TournamentRegistrationIden::PaymentStatus,
                TournamentRegistrationIden::RegistrationDate,
                TournamentRegistrationIden::ApprovalDate,
                TournamentRegistrationIden::PaymentDate,
                TournamentRegistrationIden::PaymentAmount,
                TournamentRegistrationIden::PaymentReference,
                TournamentRegistrationIden::Notes,
                TournamentRegistrationIden::Metadata,
                TournamentRegistrationIden::CreatedAt,
                TournamentRegistrationIden::UpdatedAt,
            ])
            .from(TournamentRegistrationIden::Table)
            .and_where(Expr::col(TournamentRegistrationIden::Id).eq(registration_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentRegistrationRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentRegistration::from))
    }

    async fn get_by_tournament_category(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<TournamentRegistration>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentRegistrationIden::Id,
                TournamentRegistrationIden::TournamentCategoryId,
                TournamentRegistrationIden::TeamId,
                TournamentRegistrationIden::PlayerId,
                TournamentRegistrationIden::PartnerPlayerId,
                TournamentRegistrationIden::RegistrationStatus,
                TournamentRegistrationIden::PaymentStatus,
                TournamentRegistrationIden::RegistrationDate,
                TournamentRegistrationIden::ApprovalDate,
                TournamentRegistrationIden::PaymentDate,
                TournamentRegistrationIden::PaymentAmount,
                TournamentRegistrationIden::PaymentReference,
                TournamentRegistrationIden::Notes,
                TournamentRegistrationIden::Metadata,
                TournamentRegistrationIden::CreatedAt,
                TournamentRegistrationIden::UpdatedAt,
            ])
            .from(TournamentRegistrationIden::Table)
            .and_where(Expr::col(TournamentRegistrationIden::TournamentCategoryId).eq(category_id))
            .order_by(
                TournamentRegistrationIden::RegistrationDate,
                sea_query::Order::Desc,
            )
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentRegistrationRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(TournamentRegistration::from).collect())
    }

    async fn get_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, AppError> {
        let sql = r#"
            SELECT 
                tr.id,
                tr.tournament_category_id,
                t.name as tournament_name,
                tc.name as category_name,
                tm.name as team_name,
                p1.name as player_name,
                p2.name as partner_name,
                tr.registration_status,
                tr.payment_status,
                tr.registration_date
            FROM tournament_registrations tr
            INNER JOIN tournament_categories tc ON tc.id = tr.tournament_category_id
            INNER JOIN tournaments t ON t.id = tc.tournament_id
            LEFT JOIN teams tm ON tm.id = tr.team_id
            LEFT JOIN players p1 ON p1.id = tr.player_id
            LEFT JOIN players p2 ON p2.id = tr.partner_player_id
            WHERE t.id = $1
            ORDER BY tr.registration_date DESC
        "#;

        let rows: Vec<RegistrationWithDetailsRow> = sqlx::query_as(sql)
            .bind(tournament_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(RegistrationWithDetails::from)
            .collect())
    }

    async fn get_by_player(
        &self,
        player_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, AppError> {
        let sql = r#"
            SELECT 
                tr.id,
                tr.tournament_category_id,
                t.name as tournament_name,
                tc.name as category_name,
                tm.name as team_name,
                p1.name as player_name,
                p2.name as partner_name,
                tr.registration_status,
                tr.payment_status,
                tr.registration_date
            FROM tournament_registrations tr
            INNER JOIN tournament_categories tc ON tc.id = tr.tournament_category_id
            INNER JOIN tournaments t ON t.id = tc.tournament_id
            LEFT JOIN teams tm ON tm.id = tr.team_id
            LEFT JOIN players p1 ON p1.id = tr.player_id
            LEFT JOIN players p2 ON p2.id = tr.partner_player_id
            WHERE tr.player_id = $1 OR tr.partner_player_id = $1
            ORDER BY tr.registration_date DESC
        "#;

        let rows: Vec<RegistrationWithDetailsRow> = sqlx::query_as(sql)
            .bind(player_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(RegistrationWithDetails::from)
            .collect())
    }

    async fn get_by_team(&self, team_id: Uuid) -> Result<Vec<RegistrationWithDetails>, AppError> {
        let sql = r#"
            SELECT 
                tr.id,
                tr.tournament_category_id,
                t.name as tournament_name,
                tc.name as category_name,
                tm.name as team_name,
                p1.name as player_name,
                p2.name as partner_name,
                tr.registration_status,
                tr.payment_status,
                tr.registration_date
            FROM tournament_registrations tr
            INNER JOIN tournament_categories tc ON tc.id = tr.tournament_category_id
            INNER JOIN tournaments t ON t.id = tc.tournament_id
            LEFT JOIN teams tm ON tm.id = tr.team_id
            LEFT JOIN players p1 ON p1.id = tr.player_id
            LEFT JOIN players p2 ON p2.id = tr.partner_player_id
            WHERE tr.team_id = $1
            ORDER BY tr.registration_date DESC
        "#;

        let rows: Vec<RegistrationWithDetailsRow> = sqlx::query_as(sql)
            .bind(team_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(RegistrationWithDetails::from)
            .collect())
    }

    async fn update(
        &self,
        registration_id: Uuid,
        registration_data: EditableTournamentRegistration,
    ) -> Result<Option<TournamentRegistration>, AppError> {
        let mut query = Query::update();
        query.table(TournamentRegistrationIden::Table);

        if let Some(registration_status) = registration_data.registration_status {
            query.value(
                TournamentRegistrationIden::RegistrationStatus,
                registration_status_to_string(registration_status),
            );
            if matches!(registration_status, RegistrationStatus::Approved) {
                query.value(TournamentRegistrationIden::ApprovalDate, Utc::now());
            }
        }
        if let Some(payment_status) = registration_data.payment_status {
            query.value(
                TournamentRegistrationIden::PaymentStatus,
                payment_status_to_string(payment_status),
            );
            if matches!(payment_status, PaymentStatus::Completed) {
                query.value(TournamentRegistrationIden::PaymentDate, Utc::now());
            }
        }
        if let Some(payment_amount) = registration_data.payment_amount {
            query.value(TournamentRegistrationIden::PaymentAmount, payment_amount);
        }
        if let Some(payment_reference) = registration_data.payment_reference {
            query.value(
                TournamentRegistrationIden::PaymentReference,
                payment_reference,
            );
        }
        if let Some(notes) = registration_data.notes {
            query.value(TournamentRegistrationIden::Notes, notes);
        }
        if let Some(metadata) = registration_data.metadata {
            query.value(TournamentRegistrationIden::Metadata, metadata);
        }

        query.value(TournamentRegistrationIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(TournamentRegistrationIden::Id).eq(registration_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentRegistrationRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentRegistration::from))
    }

    async fn delete(
        &self,
        registration_id: Uuid,
    ) -> Result<Option<TournamentRegistration>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TournamentRegistrationIden::Table)
            .and_where(Expr::col(TournamentRegistrationIden::Id).eq(registration_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentRegistrationRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentRegistration::from))
    }
}
