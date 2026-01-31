use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::participant::{
    CreatePlayer, EditablePlayer, Player, PlayerRepository,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden ====================

pub enum PlayerIden {
    Table,
    Id,
    Name,
    UserId,
    CreatedAt,
}

impl Iden for PlayerIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                PlayerIden::Table => "players",
                PlayerIden::Id => "id",
                PlayerIden::Name => "name",
                PlayerIden::UserId => "user_id",
                PlayerIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct PlayerRow {
    id: Uuid,
    name: String,
    user_id: Option<Uuid>,
    created_at: chrono::DateTime<Utc>,
}

impl From<PlayerRow> for Player {
    fn from(row: PlayerRow) -> Self {
        Player {
            id: row.id,
            name: row.name,
            user_id: row.user_id,
            created_at: row.created_at,
        }
    }
}

// ==================== Repository ====================

pub struct PgPlayerRepository {
    pool: DbPool,
}

impl PgPlayerRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlayerRepository for PgPlayerRepository {
    async fn find_all(&self) -> Result<Vec<Player>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                PlayerIden::Id,
                PlayerIden::Name,
                PlayerIden::UserId,
                PlayerIden::CreatedAt,
            ])
            .from(PlayerIden::Table)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<PlayerRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Player::from).collect())
    }

    async fn find_by_id(&self, player_id: Uuid) -> Result<Option<Player>, AppError> {
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

        let row: Option<PlayerRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Player::from))
    }

    async fn create(&self, new_player: CreatePlayer) -> Result<Player, AppError> {
        let (sql, values) = Query::insert()
            .into_table(PlayerIden::Table)
            .columns([PlayerIden::Name, PlayerIden::UserId])
            .values_panic([new_player.name.into(), new_player.user_id.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: PlayerRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(Player::from(row))
    }

    async fn update(
        &self,
        player_id: Uuid,
        player_data: EditablePlayer,
    ) -> Result<Option<Player>, AppError> {
        let (sql, values) = Query::update()
            .table(PlayerIden::Table)
            .values([
                (PlayerIden::Name, player_data.name.into()),
                (PlayerIden::UserId, player_data.user_id.into()),
            ])
            .and_where(Expr::col(PlayerIden::Id).eq(player_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<PlayerRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Player::from))
    }

    async fn delete(&self, player_id: Uuid) -> Result<Option<Player>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(PlayerIden::Table)
            .and_where(Expr::col(PlayerIden::Id).eq(player_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<PlayerRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Player::from))
    }
}
