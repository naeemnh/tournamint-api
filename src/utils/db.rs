use sqlx::{Error, Pool, Postgres, Transaction};
use std::{future::Future, pin::Pin};

pub async fn with_transaction<'a, F, R>(pool: &'a Pool<Postgres>, f: F) -> Result<R, Error>
where
    F: for<'t> FnOnce(
        &'t mut Transaction<'_, Postgres>,
    ) -> Pin<Box<dyn Future<Output = Result<R, Error>> + 't>>,
{
    let mut tx = pool.begin().await?;
    match f(&mut tx).await {
        Ok(result) => {
            tx.commit().await?;
            Ok(result)
        }
        Err(e) => {
            tx.rollback().await?;
            Err(e)
        }
    }
}
