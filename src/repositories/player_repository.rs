use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::player::{CreatePlayer, EditablePlayer, Player, PlayerIden};

pub struct PlayerRepository;

impl PlayerRepository {
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

    pub async fn find_by_id(tx: &mut PgConnection, id: Uuid) -> Result<Option<Player>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                PlayerIden::Id,
                PlayerIden::Name,
                PlayerIden::UserId,
                PlayerIden::CreatedAt,
            ])
            .from(PlayerIden::Table)
            .and_where(Expr::col(PlayerIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn create(tx: &mut PgConnection, data: CreatePlayer) -> Result<Player, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(PlayerIden::Table)
            .columns([PlayerIden::Name, PlayerIden::UserId])
            .values_panic([data.name.into(), data.user_id.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        data: EditablePlayer,
    ) -> Result<Option<Player>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(PlayerIden::Table)
            .values([
                (PlayerIden::Name, data.name.into()),
                (PlayerIden::UserId, data.user_id.into()),
            ])
            .and_where(Expr::col(PlayerIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn delete(tx: &mut PgConnection, id: Uuid) -> Result<Option<Player>, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(PlayerIden::Table)
            .and_where(Expr::col(PlayerIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }
}
