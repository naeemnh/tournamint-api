use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::user::{CreateUser, EditableUser, User, UserIden},
};

pub async fn find_all(pool: &DbPool) -> Result<Vec<User>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, _) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Email,
            UserIden::CreatedAt,
        ])
        .from(UserIden::Table)
        .build_sqlx(PostgresQueryBuilder);

    let users = sqlx::query_as(&sql).fetch_all(&mut *tx).await?;

    tx.commit().await?;
    Ok(users)
}

pub async fn create(pool: &DbPool, new_user: CreateUser) -> Result<User, sqlx::Error> {
    // Use a transaction to prevent prepared statement issues
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([UserIden::Username, UserIden::Email])
        .values([new_user.username.into(), new_user.email.into()])
        .unwrap()
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with(&sql, values)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn find_by_id(pool: &DbPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    // Use explicit type annotations for PostgreSQL
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Email,
            UserIden::CreatedAt,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(user_id))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn update(
    pool: &DbPool,
    user_id: Uuid,
    user_data: EditableUser,
) -> Result<Option<User>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::update()
        .table(UserIden::Table)
        .values([
            (UserIden::Username, user_data.username.into()),
            (UserIden::Email, user_data.email.into()),
        ])
        .and_where(Expr::col(UserIden::Id).eq(user_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn delete(pool: &DbPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let (sql, values) = Query::delete()
        .from_table(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(user_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(result)
}
