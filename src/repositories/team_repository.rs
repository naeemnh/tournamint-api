use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::team::{EditableTeam, NewTeam, Team, TeamIden},
};

pub async fn find_all(pool: &DbPool) -> Result<Vec<Team>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, _) = Query::select()
        .columns([TeamIden::Id, TeamIden::Name, TeamIden::CreatedAt])
        .from(TeamIden::Table)
        .build_sqlx(PostgresQueryBuilder);

    let teams = sqlx::query_as(&sql).fetch_all(&mut *tx).await?;

    tx.commit().await?;
    Ok(teams)
}

pub async fn create(pool: &DbPool, new_team: NewTeam) -> Result<Option<Team>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::insert()
        .into_table(TeamIden::Table)
        .columns([TeamIden::Name])
        .values_panic([new_team.name.into()])
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let team = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(team)
}

pub async fn find_by_id(pool: &DbPool, team_id: Uuid) -> Result<Option<Team>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::select()
        .columns([TeamIden::Id, TeamIden::Name, TeamIden::CreatedAt])
        .from(TeamIden::Table)
        .and_where(Expr::col(TeamIden::Id).eq(team_id))
        .build_sqlx(PostgresQueryBuilder);

    let team = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(team)
}

pub async fn update(
    pool: &DbPool,
    team_id: Uuid,
    team_data: EditableTeam,
) -> Result<Option<Team>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::update()
        .table(TeamIden::Table)
        .values([(TeamIden::Name, team_data.name.into())])
        .and_where(Expr::col(TeamIden::Id).eq(team_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let team = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(team)
}

pub async fn delete(pool: &DbPool, team_id: Uuid) -> Result<Option<Team>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::delete()
        .from_table(TeamIden::Table)
        .and_where(Expr::col(TeamIden::Id).eq(team_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(result)
}
