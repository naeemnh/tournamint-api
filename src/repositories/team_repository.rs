use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::{
    player::PlayerIden,
    team::{EditableTeam, NewTeam, Team, TeamIden, TeamWithMembers},
    team_member::{TeamMemberIden, TeamPlayer},
};

pub struct TeamRepository;

impl TeamRepository {
    pub async fn find_all(tx: &mut PgConnection) -> Result<Vec<Team>, sqlx::Error> {
        let (sql, _) = Query::select()
            .columns([TeamIden::Id, TeamIden::Name, TeamIden::CreatedAt])
            .from(TeamIden::Table)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as(&sql).fetch_all(&mut *tx).await
    }

    pub async fn create(tx: &mut PgConnection, new_team: NewTeam) -> Result<Option<Team>, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(TeamIden::Table)
            .columns([TeamIden::Name])
            .values_panic([new_team.name.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn find_by_id(
        tx: &mut PgConnection,
        team_id: Uuid,
    ) -> Result<Option<TeamWithMembers>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([TeamIden::Id, TeamIden::Name, TeamIden::CreatedAt])
            .from(TeamIden::Table)
            .and_where(Expr::col(TeamIden::Id).eq(team_id))
            .build_sqlx(PostgresQueryBuilder);

        let team = sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await?;

        let (members_sql, members_values) = Query::select()
            .columns([
                (PlayerIden::Table, PlayerIden::Id),
                (PlayerIden::Table, PlayerIden::Name),
                (PlayerIden::Table, PlayerIden::UserId),
            ])
            .columns([
                (TeamMemberIden::Table, TeamMemberIden::IsCaptain),
                (TeamMemberIden::Table, TeamMemberIden::JerseyNumber),
                (TeamMemberIden::Table, TeamMemberIden::JoinedAt),
            ])
            .from(TeamMemberIden::Table)
            .inner_join(
                PlayerIden::Table,
                Expr::col((TeamMemberIden::Table, TeamMemberIden::PlayerId))
                    .equals((PlayerIden::Table, PlayerIden::Id)),
            )
            .and_where(Expr::col(TeamMemberIden::TeamId).eq(team_id))
            .build_sqlx(PostgresQueryBuilder);

        let members: Vec<TeamPlayer> = sqlx::query_as_with(&members_sql, members_values)
            .fetch_all(&mut *tx)
            .await?
            .into();

        Ok(team.map(|team| TeamWithMembers { team, members }))
    }

    pub async fn update(
        tx: &mut PgConnection,
        team_id: Uuid,
        team_data: EditableTeam,
    ) -> Result<Option<Team>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(TeamIden::Table)
            .values([(TeamIden::Name, team_data.name.into())])
            .and_where(Expr::col(TeamIden::Id).eq(team_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn delete(tx: &mut PgConnection, team_id: Uuid) -> Result<Option<Team>, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TeamIden::Table)
            .and_where(Expr::col(TeamIden::Id).eq(team_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }
}