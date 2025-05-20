use sea_query::{error::Error as SeaQueryError, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Error as SqlxError, PgConnection};

use crate::models::token::{Token, TokenIden};

pub async fn upsert_refresh_token(
    tx: &mut PgConnection,
    token_data: Token,
) -> Result<(), sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(TokenIden::Table)
        .columns([
            TokenIden::UserId,
            TokenIden::RefreshToken,
            TokenIden::ExpiresAt,
        ])
        .values([
            token_data.user_id.into(),
            token_data.refresh_token.into(),
            token_data.expires_at.into(),
        ])
        .map_err(|e: SeaQueryError| SqlxError::Configuration(Box::new(e)))?
        .on_conflict(
            sea_query::OnConflict::column(TokenIden::UserId)
                .update_columns([TokenIden::RefreshToken, TokenIden::ExpiresAt])
                .to_owned(),
        )
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    Ok(())
}
