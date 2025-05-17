use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::player::{CreatePlayer, EditablePlayer, Player, PlayerIden};

pub async fn find_all(tx: &mut PgConnection) -> Result<Vec<Player>, sqlx::Error> {
    let (sql, _) = Query::select()
        .columns([
            PlayerIden::Id,
            PlayerIden::Name,
            PlayerIden::UserId,
            PlayerIden::CreatedAt,
        ])
        .from(PlayerIden::Table)
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as(&sql).fetch_all(&mut *tx).await
}

pub async fn create(
    tx: &mut PgConnection,
    new_player: CreatePlayer,
) -> Result<Player, sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(PlayerIden::Table)
        .columns([PlayerIden::Name, PlayerIden::UserId])
        .values_panic([new_player.name.into(), new_player.user_id.into()])
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
}

pub async fn find_by_id(
    tx: &mut PgConnection,
    player_id: Uuid,
) -> Result<Option<Player>, sqlx::Error> {
    let (sql, values) = Query::select()
        .columns([
            PlayerIden::Id,
            PlayerIden::Name,
            PlayerIden::UserId,
            PlayerIden::CreatedAt,
        ])
        .from(PlayerIden::Table)
        .and_where(Expr::col(PlayerIden::Id).eq(player_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await
}

pub async fn update(
    tx: &mut PgConnection,
    player_id: Uuid,
    player_data: EditablePlayer,
) -> Result<Option<Player>, sqlx::Error> {
    let (sql, values) = Query::update()
        .table(PlayerIden::Table)
        .values([
            (PlayerIden::Name, player_data.name.into()),
            (PlayerIden::UserId, player_data.user_id.into()),
        ])
        .and_where(Expr::col(PlayerIden::Id).eq(player_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await
}

pub async fn delete(tx: &mut PgConnection, player_id: Uuid) -> Result<Option<Player>, sqlx::Error> {
    let (sql, values) = Query::delete()
        .from_table(PlayerIden::Table)
        .and_where(Expr::col(PlayerIden::Id).eq(player_id))
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await
}
