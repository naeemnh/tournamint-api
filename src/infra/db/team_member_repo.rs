use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::participant::{
    EditableTeamMember, NewTeamMember, TeamMember, TeamMemberRepository, TeamPlayer,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden ====================

pub enum TeamMemberIden {
    Table,
    TeamId,
    PlayerId,
    IsCaptain,
    JerseyNumber,
    JoinedAt,
}

impl Iden for TeamMemberIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TeamMemberIden::Table => "team_members",
                TeamMemberIden::TeamId => "team_id",
                TeamMemberIden::PlayerId => "player_id",
                TeamMemberIden::IsCaptain => "is_captain",
                TeamMemberIden::JerseyNumber => "jersey_number",
                TeamMemberIden::JoinedAt => "joined_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TeamMemberRow {
    team_id: Uuid,
    player_id: Uuid,
    is_captain: bool,
    jersey_number: Option<i32>,
    joined_at: chrono::DateTime<Utc>,
}

impl From<TeamMemberRow> for TeamMember {
    fn from(row: TeamMemberRow) -> Self {
        TeamMember {
            team_id: row.team_id,
            player_id: row.player_id,
            is_captain: row.is_captain,
            jersey_number: row.jersey_number,
            joined_at: row.joined_at,
        }
    }
}

/// Row from join of team_members + players for get_by_team
#[derive(Debug, FromRow)]
struct TeamPlayerRow {
    id: Uuid,
    name: String,
    user_id: Option<Uuid>,
    is_captain: bool,
    jersey_number: Option<i32>,
    joined_at: chrono::DateTime<Utc>,
}

impl From<TeamPlayerRow> for TeamPlayer {
    fn from(row: TeamPlayerRow) -> Self {
        TeamPlayer {
            id: row.id,
            name: row.name,
            user_id: row.user_id,
            is_captain: row.is_captain,
            jersey_number: row.jersey_number,
            joined_at: row.joined_at,
        }
    }
}

// ==================== Repository ====================

pub struct PgTeamMemberRepository {
    pool: DbPool,
}

impl PgTeamMemberRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamMemberRepository for PgTeamMemberRepository {
    async fn create(&self, new_member: NewTeamMember) -> Result<TeamMember, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TeamMemberIden::Table)
            .columns([
                TeamMemberIden::TeamId,
                TeamMemberIden::PlayerId,
                TeamMemberIden::IsCaptain,
                TeamMemberIden::JerseyNumber,
            ])
            .values_panic([
                new_member.team_id.into(),
                new_member.player_id.into(),
                new_member.is_captain.unwrap_or(false).into(),
                new_member.jersey_number.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TeamMemberRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(TeamMember::from(row))
    }

    async fn get_by_team(&self, team_id: Uuid) -> Result<Vec<TeamPlayer>, AppError> {
        let sql_raw = r#"
            SELECT p.id, p.name, p.user_id, tm.is_captain, tm.jersey_number, tm.joined_at
            FROM team_members tm
            INNER JOIN players p ON p.id = tm.player_id
            WHERE tm.team_id = $1
        "#;
        let rows: Vec<TeamPlayerRow> =
            sqlx::query_as(sql_raw).bind(team_id).fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(TeamPlayer::from).collect())
    }

    async fn get_by_id(
        &self,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<Option<TeamMember>, AppError> {
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

        let row: Option<TeamMemberRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TeamMember::from))
    }

    async fn update(
        &self,
        team_id: Uuid,
        player_id: Uuid,
        member_data: EditableTeamMember,
    ) -> Result<Option<TeamMember>, AppError> {
        let (sql, values) = Query::update()
            .table(TeamMemberIden::Table)
            .values([
                (TeamMemberIden::IsCaptain, member_data.is_captain.into()),
                (TeamMemberIden::JerseyNumber, member_data.jersey_number.into()),
            ])
            .and_where(Expr::col(TeamMemberIden::TeamId).eq(team_id))
            .and_where(Expr::col(TeamMemberIden::PlayerId).eq(player_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TeamMemberRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TeamMember::from))
    }

    async fn delete(
        &self,
        team_id: Uuid,
        player_id: Uuid,
    ) -> Result<Option<TeamMember>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TeamMemberIden::Table)
            .and_where(Expr::col(TeamMemberIden::TeamId).eq(team_id))
            .and_where(Expr::col(TeamMemberIden::PlayerId).eq(player_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TeamMemberRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TeamMember::from))
    }
}
