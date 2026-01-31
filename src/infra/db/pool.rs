use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub struct DbConfig;

impl DbConfig {
    pub async fn create_db_pool() -> Result<DbPool, sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file or environment variables");

        PgPoolOptions::new()
            .min_connections(1)
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(30))
            .test_before_acquire(true)
            .connect(&database_url)
            .await
    }
}

/// Helper function to run operations within a transaction
pub async fn with_transaction<F, T, Fut>(pool: &DbPool, f: F) -> Result<T, sqlx::Error>
where
    F: FnOnce(&mut sqlx::PgConnection) -> Fut,
    Fut: std::future::Future<Output = Result<T, sqlx::Error>>,
{
    let mut conn = pool.acquire().await?;
    f(&mut conn).await
}
