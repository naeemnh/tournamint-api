use uuid::Uuid;

use crate::{
    config::DbPool,
    models::user::{CreateUser, EditableUser, User},
};

pub async fn find_all(pool: &DbPool) -> Result<Vec<User>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let users = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, created_at
            FROM users"#,
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(users)
}

pub async fn create(pool: &DbPool, new_user: CreateUser) -> Result<User, sqlx::Error> {
    // Use a transaction to prevent prepared statement issues
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as!(
        User,
        r#"INSERT INTO users (id, username, email)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, created_at"#,
        Uuid::new_v4(),
        new_user.username,
        new_user.email
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn find_by_id(pool: &DbPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    // Use explicit type annotations for PostgreSQL
    let mut tx = pool.begin().await?;

    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, created_at 
            FROM users 
            WHERE id = $1"#,
        user_id,
    )
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

    let user = sqlx::query_as!(
        User,
        r#"UPDATE users 
        SET username=$1, email=$2 WHERE id=$3
        RETURNING id, username, email, created_at"#,
        user_data.username,
        user_data.email,
        user_id,
    )
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user)
}

pub async fn delete(pool: &DbPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query_as!(
        User,
        r#"DELETE FROM users
            WHERE id = $1 RETURNING id, username, email, created_at"#,
        user_id,
    )
    .fetch_optional(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(result)
}
