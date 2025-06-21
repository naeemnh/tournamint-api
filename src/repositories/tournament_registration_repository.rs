use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::tournament_registration::{
    EditableTournamentRegistration, NewTournamentRegistration, PaymentStatus, RegistrationStatus,
    RegistrationWithDetails, TournamentRegistration, TournamentRegistrationIden,
};

pub struct TournamentRegistrationRepository;

impl TournamentRegistrationRepository {
    pub async fn create(
        tx: &mut PgConnection,
        data: NewTournamentRegistration,
    ) -> Result<TournamentRegistration, sqlx::Error> {
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
                data.tournament_category_id.into(),
                data.team_id.into(),
                data.player_id.into(),
                data.partner_player_id.into(),
                data.notes.into(),
                data.metadata.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_id(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<TournamentRegistration, sqlx::Error> {
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
            .and_where(Expr::col(TournamentRegistrationIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_tournament_category(
        tx: &mut PgConnection,
        tournament_category_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, sqlx::Error> {
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
            WHERE tr.tournament_category_id = $1
            ORDER BY tr.registration_date DESC
        "#;

        sqlx::query_as(sql)
            .bind(tournament_category_id)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn get_by_tournament(
        tx: &mut PgConnection,
        tournament_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, sqlx::Error> {
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

        sqlx::query_as(sql)
            .bind(tournament_id)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn get_by_player(
        tx: &mut PgConnection,
        player_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, sqlx::Error> {
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

        sqlx::query_as(sql)
            .bind(player_id)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn get_by_team(
        tx: &mut PgConnection,
        team_id: Uuid,
    ) -> Result<Vec<RegistrationWithDetails>, sqlx::Error> {
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

        sqlx::query_as(sql).bind(team_id).fetch_all(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        data: EditableTournamentRegistration,
    ) -> Result<TournamentRegistration, sqlx::Error> {
        let mut query = Query::update();
        query.table(TournamentRegistrationIden::Table);

        if let Some(registration_status) = data.registration_status {
            query.value(
                TournamentRegistrationIden::RegistrationStatus,
                serde_json::to_string(&registration_status).unwrap(),
            );
            if matches!(registration_status, RegistrationStatus::Approved) {
                query.value(TournamentRegistrationIden::ApprovalDate, Utc::now());
            }
        }

        if let Some(payment_status) = data.payment_status {
            query.value(
                TournamentRegistrationIden::PaymentStatus,
                serde_json::to_string(&payment_status).unwrap(),
            );
            if matches!(payment_status, PaymentStatus::Completed) {
                query.value(TournamentRegistrationIden::PaymentDate, Utc::now());
            }
        }

        if let Some(payment_amount) = data.payment_amount {
            query.value(TournamentRegistrationIden::PaymentAmount, payment_amount);
        }

        if let Some(payment_reference) = data.payment_reference {
            query.value(
                TournamentRegistrationIden::PaymentReference,
                payment_reference,
            );
        }

        if let Some(notes) = data.notes {
            query.value(TournamentRegistrationIden::Notes, notes);
        }

        if let Some(metadata) = data.metadata {
            query.value(TournamentRegistrationIden::Metadata, metadata);
        }

        query.value(TournamentRegistrationIden::UpdatedAt, Utc::now());

        let (sql, values) = query
            .and_where(Expr::col(TournamentRegistrationIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<TournamentRegistration, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TournamentRegistrationIden::Table)
            .and_where(Expr::col(TournamentRegistrationIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }
}
