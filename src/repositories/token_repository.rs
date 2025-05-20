use sea_query::{error::Error as SeaQueryError, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Error as SqlxError, PgConnection};

use crate::models::token::{UserToken, UserTokenIden};

pub async fn upsert_refresh_token(
    tx: &mut PgConnection,
    token_data: UserToken,
) -> Result<(), sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(UserTokenIden::Table)
        .columns([
            UserTokenIden::UserId,
            UserTokenIden::RefreshToken,
            UserTokenIden::ExpiresAt,
        ])
        .values([
            token_data.user_id.into(),
            token_data.refresh_token.into(),
            token_data.expires_at.into(),
        ])
        .map_err(|e: SeaQueryError| SqlxError::Configuration(Box::new(e)))?
        .on_conflict(
            sea_query::OnConflict::column(UserTokenIden::UserId)
                .update_columns([UserTokenIden::RefreshToken, UserTokenIden::ExpiresAt])
                .to_owned(),
        )
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    Ok(())
}
