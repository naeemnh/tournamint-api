use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::tournament_category::{
    EditableTournamentCategory, NewTournamentCategory, TournamentCategory, TournamentCategoryIden,
};

pub struct TournamentCategoryRepository;

impl TournamentCategoryRepository {
    pub async fn create(
        tx: &mut PgConnection,
        data: NewTournamentCategory,
    ) -> Result<TournamentCategory, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(TournamentCategoryIden::Table)
            .columns([
                TournamentCategoryIden::TournamentId,
                TournamentCategoryIden::Name,
                TournamentCategoryIden::Description,
                TournamentCategoryIden::TeamComposition,
                TournamentCategoryIden::MinParticipants,
                TournamentCategoryIden::MaxParticipants,
                TournamentCategoryIden::EntryFee,
                TournamentCategoryIden::PrizeDistribution,
                TournamentCategoryIden::Rules,
                TournamentCategoryIden::Constraints,
            ])
            .values_panic([
                data.tournament_id.into(),
                data.name.into(),
                data.description.into(),
                serde_json::to_string(&data.team_composition)
                    .unwrap()
                    .into(),
                data.min_participants.unwrap_or(2).into(),
                data.max_participants.into(),
                data.entry_fee.into(),
                data.prize_distribution.into(),
                data.rules.into(),
                data.constraints.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_tournament(
        tx: &mut PgConnection,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentCategory>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                TournamentCategoryIden::Id,
                TournamentCategoryIden::TournamentId,
                TournamentCategoryIden::Name,
                TournamentCategoryIden::Description,
                TournamentCategoryIden::TeamComposition,
                TournamentCategoryIden::MinParticipants,
                TournamentCategoryIden::MaxParticipants,
                TournamentCategoryIden::EntryFee,
                TournamentCategoryIden::PrizeDistribution,
                TournamentCategoryIden::Rules,
                TournamentCategoryIden::Constraints,
                TournamentCategoryIden::CreatedAt,
                TournamentCategoryIden::UpdatedAt,
            ])
            .from(TournamentCategoryIden::Table)
            .and_where(Expr::col(TournamentCategoryIden::TournamentId).eq(tournament_id))
            .order_by(TournamentCategoryIden::Name, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_by_id(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<TournamentCategory, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                TournamentCategoryIden::Id,
                TournamentCategoryIden::TournamentId,
                TournamentCategoryIden::Name,
                TournamentCategoryIden::Description,
                TournamentCategoryIden::TeamComposition,
                TournamentCategoryIden::MinParticipants,
                TournamentCategoryIden::MaxParticipants,
                TournamentCategoryIden::EntryFee,
                TournamentCategoryIden::PrizeDistribution,
                TournamentCategoryIden::Rules,
                TournamentCategoryIden::Constraints,
                TournamentCategoryIden::CreatedAt,
                TournamentCategoryIden::UpdatedAt,
            ])
            .from(TournamentCategoryIden::Table)
            .and_where(Expr::col(TournamentCategoryIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        data: EditableTournamentCategory,
    ) -> Result<TournamentCategory, sqlx::Error> {
        let mut query = Query::update();
        query.table(TournamentCategoryIden::Table);

        if let Some(name) = data.name {
            query.value(TournamentCategoryIden::Name, name);
        }
        if let Some(description) = data.description {
            query.value(TournamentCategoryIden::Description, description);
        }
        if let Some(team_composition) = data.team_composition {
            query.value(
                TournamentCategoryIden::TeamComposition,
                serde_json::to_string(&team_composition).unwrap(),
            );
        }
        if let Some(min_participants) = data.min_participants {
            query.value(TournamentCategoryIden::MinParticipants, min_participants);
        }
        if let Some(max_participants) = data.max_participants {
            query.value(TournamentCategoryIden::MaxParticipants, max_participants);
        }
        if let Some(entry_fee) = data.entry_fee {
            query.value(TournamentCategoryIden::EntryFee, entry_fee);
        }
        if let Some(prize_distribution) = data.prize_distribution {
            query.value(
                TournamentCategoryIden::PrizeDistribution,
                prize_distribution,
            );
        }
        if let Some(rules) = data.rules {
            query.value(TournamentCategoryIden::Rules, rules);
        }
        if let Some(constraints) = data.constraints {
            query.value(TournamentCategoryIden::Constraints, constraints);
        }

        query.value(TournamentCategoryIden::UpdatedAt, Utc::now());

        let (sql, values) = query
            .and_where(Expr::col(TournamentCategoryIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<TournamentCategory, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TournamentCategoryIden::Table)
            .and_where(Expr::col(TournamentCategoryIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }
}
