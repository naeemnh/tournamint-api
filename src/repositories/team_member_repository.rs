use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::{
    player::PlayerIden,
    team_member::{EditableTeamMember, NewTeamMember, TeamMember, TeamMemberIden, TeamPlayer},
};

pub struct TeamMemberRepository;

impl TeamMemberRepository {
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

    pub async fn get_by_team(
        tx: &mut PgConnection,
        team_id: Uuid,
    ) -> Result<Vec<TeamPlayer>, sqlx::Error> {
        let (sql, values) = Query::select()
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
            .and_where(Expr::col((TeamMemberIden::Table, TeamMemberIden::TeamId)).eq(team_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_by_id(
        tx: &mut PgConnection,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<TeamMember, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                TeamMemberIden::TeamId,
                TeamMemberIden::PlayerId,
                TeamMemberIden::IsCaptain,
                TeamMemberIden::JerseyNumber,
                TeamMemberIden::JoinedAt,
            ])
            .from(TeamMemberIden::Table)
            .and_where(Expr::col(TeamMemberIden::TeamId).eq(team_id))
            .and_where(Expr::col(TeamMemberIden::PlayerId).eq(player_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        team_id: Uuid,
        player_id: Uuid,
        data: EditableTeamMember,
    ) -> Result<TeamMember, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(TeamMemberIden::Table)
            .values([
                (TeamMemberIden::IsCaptain, data.is_captain.into()),
                (TeamMemberIden::JerseyNumber, data.jersey_number.into()),
            ])
            .and_where(Expr::col(TeamMemberIden::TeamId).eq(team_id))
            .and_where(Expr::col(TeamMemberIden::PlayerId).eq(player_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete(
        tx: &mut PgConnection,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<TeamMember, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TeamMemberIden::Table)
            .and_where(Expr::col(TeamMemberIden::TeamId).eq(team_id))
            .and_where(Expr::col(TeamMemberIden::PlayerId).eq(player_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }
}
