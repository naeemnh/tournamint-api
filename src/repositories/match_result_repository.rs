use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;
use crate::models::match_result::{MatchResult, NewMatchResult, EditableMatchResult, MatchScoreSummary, MatchResultIden};

pub struct MatchResultRepository;

impl MatchResultRepository {

    pub async fn create(
        tx: &mut PgConnection,
        request: NewMatchResult,
    ) -> Result<MatchResult, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(MatchResultIden::Table)
            .columns([
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
            ])
            .values_panic([
                request.match_id.into(),
                request.set_number.into(),
                request.participant1_score.into(),
                request.participant2_score.into(),
                request.period_number.into(),
                request.period_name.into(),
                request.scoring_data.into(),
                request.participant1_stats.into(),
                request.participant2_stats.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn find_by_id(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<MatchResult>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                MatchResultIden::Id,
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
                MatchResultIden::CreatedAt,
                MatchResultIden::UpdatedAt,
            ])
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        request: EditableMatchResult,
    ) -> Result<MatchResult, sqlx::Error> {
        let mut query = Query::update();
        query.table(MatchResultIden::Table);

        if let Some(participant1_score) = request.participant1_score {
            query.value(MatchResultIden::Participant1Score, participant1_score);
        }

        if let Some(participant2_score) = request.participant2_score {
            query.value(MatchResultIden::Participant2Score, participant2_score);
        }

        if let Some(scoring_data) = request.scoring_data {
            query.value(MatchResultIden::ScoringData, scoring_data);
        }

        if let Some(participant1_stats) = request.participant1_stats {
            query.value(MatchResultIden::Participant1Stats, participant1_stats);
        }

        if let Some(participant2_stats) = request.participant2_stats {
            query.value(MatchResultIden::Participant2Stats, participant2_stats);
        }

        query.value(MatchResultIden::UpdatedAt, Utc::now());

        let (sql, values) = query
            .and_where(Expr::col(MatchResultIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete(
        tx: &mut PgConnection,
        id: Uuid,
    ) -> Result<MatchResult, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn find_by_match(
        tx: &mut PgConnection,
        match_id: Uuid,
    ) -> Result<Vec<MatchResult>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                MatchResultIden::Id,
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
                MatchResultIden::CreatedAt,
                MatchResultIden::UpdatedAt,
            ])
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .order_by(MatchResultIden::SetNumber, sea_query::Order::Asc)
            .order_by(MatchResultIden::PeriodNumber, sea_query::Order::Asc)
            .order_by(MatchResultIden::CreatedAt, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn get_match_score_summary(
        tx: &mut PgConnection,
        match_id: Uuid,
    ) -> Result<Option<MatchScoreSummary>, sqlx::Error> {
        // Keep as raw SQL due to complex aggregation
        let sql = r#"
            SELECT 
                match_id,
                COUNT(CASE WHEN participant1_score > participant2_score THEN 1 END) as "participant1_sets_won!",
                COUNT(CASE WHEN participant2_score > participant1_score THEN 1 END) as "participant2_sets_won!",
                COALESCE(SUM(participant1_score), 0) as "participant1_total_points!",
                COALESCE(SUM(participant2_score), 0) as "participant2_total_points!"
            FROM match_results
            WHERE match_id = $1
            GROUP BY match_id
        "#;

        sqlx::query_as::<_, MatchScoreSummary>(sql)
            .bind(match_id)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn find_by_set(
        tx: &mut PgConnection,
        match_id: Uuid,
        set_number: i32,
    ) -> Result<Vec<MatchResult>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                MatchResultIden::Id,
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
                MatchResultIden::CreatedAt,
                MatchResultIden::UpdatedAt,
            ])
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .and_where(Expr::col(MatchResultIden::SetNumber).eq(set_number))
            .order_by(MatchResultIden::PeriodNumber, sea_query::Order::Asc)
            .order_by(MatchResultIden::CreatedAt, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *tx)
            .await
    }

    pub async fn delete_by_match(
        tx: &mut PgConnection,
        match_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, values)
            .execute(&mut *tx)
            .await?;
        
        Ok(result.rows_affected())
    }

    pub async fn count_by_match(
        tx: &mut PgConnection,
        match_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let (sql, values) = Query::select()
            .expr(sea_query::Expr::col(MatchResultIden::Id).count())
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .build_sqlx(PostgresQueryBuilder);

        let result: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *tx)
            .await?;
        
        Ok(result.0)
    }
}