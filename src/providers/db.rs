use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

use crate::config::DbConfig;

pub type DbPool = Pool<Postgres>;

#[derive(Clone)]
pub struct DbProvider {
    pool: DbPool,
}

impl DbProvider {
    pub async fn create_db_pool() -> Result<DbPool, Error> {
        let config = DbConfig::from_env();
        let url = config.database_url();

        let pool = PgPoolOptions::new()
            .min_connections(1)
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(30))
            .test_before_acquire(true)
            .connect(&url)
            .await?;

        Ok(pool)
    }

    pub async fn pool() -> DbPool {
        let pool = Self::create_db_pool()
            .await
            .expect("Failed to create database pool");
        pool
    }
}
