use sqlx::{Error, Pool, Postgres, Transaction};
use std::{future::Future, pin::Pin};

pub async fn with_transaction<'a, F, R>(pool: &'a Pool<Postgres>, f: F) -> Result<R, Error>
where
    F: for<'t> FnOnce(
        &'t mut Transaction<'_, Postgres>,
    ) -> Pin<Box<dyn Future<Output = Result<R, Error>> + 't>>,
{
    let mut tx = pool.begin().await?;
    let result = f(&mut tx).await;
    tx.commit().await?;
    result
}
