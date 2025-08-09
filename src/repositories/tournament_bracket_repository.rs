use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::tournament_bracket::{
    EditableTournamentBracket, NewTournamentBracket, TournamentBracket, TournamentBracketIden,
    BracketStatus,
};

pub struct TournamentBracketRepository;

impl TournamentBracketRepository {
    pub async fn create(
        tx: &mut PgConnection,
        data: NewTournamentBracket,
    ) -> Result<TournamentBracket, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(TournamentBracketIden::Table)
            .columns([
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
            ])
            .values_panic([
                data.tournament_id.into(),
                data.category_id.into(),
                serde_json::to_string(&data.bracket_type).unwrap().into(),
                data.total_rounds.into(),
                data.bracket_data.into(),
                data.settings.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_tournament_id(
        tx: &mut PgConnection,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentBracket>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                TournamentBracketIden::Id,
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::Status,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::CurrentRound,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
                TournamentBracketIden::CreatedAt,
                TournamentBracketIden::UpdatedAt,
            ])
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::TournamentId).eq(tournament_id))
            .order_by(TournamentBracketIden::CreatedAt, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_by_category_id(
        tx: &mut PgConnection,
        category_id: Uuid,
    ) -> Result<Option<TournamentBracket>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                TournamentBracketIden::Id,
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::Status,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::CurrentRound,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
                TournamentBracketIden::CreatedAt,
                TournamentBracketIden::UpdatedAt,
            ])
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::CategoryId).eq(category_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn get_by_id(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<TournamentBracket, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                TournamentBracketIden::Id,
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::Status,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::CurrentRound,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
                TournamentBracketIden::CreatedAt,
                TournamentBracketIden::UpdatedAt,
            ])
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        data: EditableTournamentBracket,
    ) -> Result<TournamentBracket, sqlx::Error> {
        let mut query = Query::update();
        query.table(TournamentBracketIden::Table);

        if let Some(status) = data.status {
            query.value(
                TournamentBracketIden::Status,
                serde_json::to_string(&status).unwrap(),
            );
        }
        if let Some(current_round) = data.current_round {
            query.value(TournamentBracketIden::CurrentRound, current_round);
        }
        if let Some(bracket_data) = data.bracket_data {
            query.value(TournamentBracketIden::BracketData, bracket_data);
        }
        if let Some(settings) = data.settings {
            query.value(TournamentBracketIden::Settings, settings);
        }

        query.value(TournamentBracketIden::UpdatedAt, Utc::now());

        let (sql, values) = query
            .and_where(Expr::col(TournamentBracketIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<TournamentBracket, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn update_status(
        tx: &mut PgConnection,
        id: Uuid,
        status: BracketStatus,
    ) -> Result<TournamentBracket, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(TournamentBracketIden::Table)
            .value(
                TournamentBracketIden::Status,
                serde_json::to_string(&status).unwrap(),
            )
            .value(TournamentBracketIden::UpdatedAt, Utc::now())
            .and_where(Expr::col(TournamentBracketIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn exists_for_tournament(
        tx: &mut PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<bool, sqlx::Error> {
        let mut query = Query::select();
        query
            .expr(Expr::cust("COUNT(*)"))
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::TournamentId).eq(tournament_id));

        if let Some(cat_id) = category_id {
            query.and_where(Expr::col(TournamentBracketIden::CategoryId).eq(cat_id));
        } else {
            query.and_where(Expr::col(TournamentBracketIden::CategoryId).is_null());
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let count: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *tx)
            .await?;

        Ok(count.0 > 0)
    }
}