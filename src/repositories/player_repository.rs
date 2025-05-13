use uuid::Uuid;

use crate::{
    config::DbPool,
    models::player::{CreatePlayer, EditablePlayer, Player},
};

pub async fn find_all(pool: &DbPool) -> Result<Vec<Player>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let players = sqlx::query_as!(
        Player,
        r#"SELECT id, name, user_id, created_at
          FROM players"#,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(players)
}

pub async fn create(pool: &DbPool, new_player: CreatePlayer) -> Result<Player, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let player = sqlx::query_as!(
        Player,
        r#"INSERT INTO players (name, user_id)
          VALUES ($1, $2)
          RETURNING id, name, user_id, created_at"#,
        new_player.name,
        new_player.user_id
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(player)
}

pub async fn find_by_id(pool: &DbPool, player_id: Uuid) -> Result<Option<Player>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let player = sqlx::query_as!(
        Player,
        r#"SELECT id, name, user_id, created_at
          FROM players
          WHERE id = $1"#,
        player_id
    )
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(player)
}

pub async fn update(
    pool: &DbPool,
    player_id: Uuid,
    player_data: EditablePlayer,
) -> Result<Option<Player>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let player = sqlx::query_as!(
        Player,
        r#"UPDATE players
    SET name=$1, user_id=$2 WHERE id=$3
    RETURNING id, name, user_id, created_at"#,
        player_data.name,
        player_data.user_id,
        player_id
    )
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(player)
}

pub async fn delete(pool: &DbPool, player_id: Uuid) -> Result<Option<Player>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query_as!(
        Player,
        r#"DELETE FROM players
          WHERE id = $1 RETURNING id, name, user_id, created_at"#,
        player_id,
    )
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(result)
}
