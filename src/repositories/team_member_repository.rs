use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;

use crate::models::team_member::{NewTeamMember, TeamMember, TeamMemberIden};

pub async fn create(
    tx: &mut PgConnection,
    player_data: NewTeamMember,
) -> Result<TeamMember, sqlx::Error> {
    let (sql, values) = Query::insert()
        .into_table(TeamMemberIden::Table)
        .columns([
            TeamMemberIden::TeamId,
            TeamMemberIden::PlayerId,
            TeamMemberIden::IsCaptain,
            TeamMemberIden::JerseyNumber,
        ])
        .values_panic([
            player_data.team_id.into(),
            player_data.player_id.into(),
            player_data.is_captain.into(),
            player_data.jersey_number.into(),
        ])
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
}
