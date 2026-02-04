use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::participant::{
    EditableTeam, NewTeam, Team, TeamRepository, TeamWithMembers, TeamPlayer,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden ====================

pub enum TeamIden {
    Table,
    Id,
    Name,
    CreatedAt,
}

impl Iden for TeamIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TeamIden::Table => "teams",
                TeamIden::Id => "id",
                TeamIden::Name => "name",
                TeamIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TeamRow {
    id: Uuid,
    name: String,
    created_at: chrono::DateTime<Utc>,
}

impl From<TeamRow> for Team {
    fn from(row: TeamRow) -> Self {
        Team {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
        }
    }
}

/// Row from join of team_members + players for find_by_id members
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

pub struct PgTeamRepository {
    pool: DbPool,
}

impl PgTeamRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TeamRepository for PgTeamRepository {
    async fn find_all(&self) -> Result<Vec<Team>, AppError> {
        let (sql, values) = Query::select()
            .columns([TeamIden::Id, TeamIden::Name, TeamIden::CreatedAt])
            .from(TeamIden::Table)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TeamRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Team::from).collect())
    }

    async fn find_by_id(&self, team_id: Uuid) -> Result<Option<TeamWithMembers>, AppError> {
        let (sql, values) = Query::select()
            .columns([TeamIden::Id, TeamIden::Name, TeamIden::CreatedAt])
            .from(TeamIden::Table)
            .and_where(Expr::col(TeamIden::Id).eq(team_id))
            .build_sqlx(PostgresQueryBuilder);

        let team_row: Option<TeamRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        let Some(team) = team_row else {
            return Ok(None);
        };

        let members_sql = r#"
            SELECT p.id, p.name, p.user_id, tm.is_captain, tm.jersey_number, tm.joined_at
            FROM team_members tm
            INNER JOIN players p ON p.id = tm.player_id
            WHERE tm.team_id = $1
        "#;
        let members: Vec<TeamPlayerRow> =
            sqlx::query_as(members_sql).bind(team_id).fetch_all(&self.pool).await?;

        let members: Vec<TeamPlayer> = members.into_iter().map(TeamPlayer::from).collect();

        Ok(Some(TeamWithMembers {
            team: Team::from(team),
            members,
        }))
    }

    async fn create(&self, new_team: NewTeam) -> Result<Team, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TeamIden::Table)
            .columns([TeamIden::Name])
            .values_panic([new_team.name.into()])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TeamRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(Team::from(row))
    }

    async fn update(
        &self,
        team_id: Uuid,
        team_data: EditableTeam,
    ) -> Result<Option<Team>, AppError> {
        let (sql, values) = Query::update()
            .table(TeamIden::Table)
            .values([(TeamIden::Name, team_data.name.into())])
            .and_where(Expr::col(TeamIden::Id).eq(team_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TeamRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Team::from))
    }

    async fn delete(&self, team_id: Uuid) -> Result<Option<Team>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TeamIden::Table)
            .and_where(Expr::col(TeamIden::Id).eq(team_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TeamRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Team::from))
    }
}
