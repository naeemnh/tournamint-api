use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::user::{CreateUser, EditableUser, User, UserIden};

pub async fn find_all(tx: &mut PgConnection) -> Result<Vec<User>, sqlx::Error> {
    let (sql, _) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Email,
            UserIden::CreatedAt,
        ])
        .from(UserIden::Table)
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as(&sql).fetch_all(tx).await
}

pub async fn create(tx: &mut PgConnection, new_user: CreateUser) -> Result<User, sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([UserIden::Username, UserIden::Email])
        .values([new_user.username.into(), new_user.email.into()])
        .unwrap()
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
}

pub async fn find_by_id(tx: &mut PgConnection, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
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

    sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await
}

pub async fn update(
    tx: &mut PgConnection,
    user_id: Uuid,
    user_data: EditableUser,
) -> Result<Option<User>, sqlx::Error> {
    let (sql, values) = Query::update()
        .table(UserIden::Table)
        .values([
            (UserIden::Username, user_data.username.into()),
            (UserIden::Email, user_data.email.into()),
        ])
        .and_where(Expr::col(UserIden::Id).eq(user_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await
}

pub async fn delete(tx: &mut PgConnection, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let (sql, values) = Query::delete()
        .from_table(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(user_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await
}
