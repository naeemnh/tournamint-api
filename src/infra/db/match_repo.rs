use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::match_domain::{
    EditableMatch, LiveMatchUpdate, Match, MatchAnalytics, MatchComment, MatchMedia,
    MatchRepository, MatchScheduleItem, MatchStatistics, MatchStatus, MatchSubscription, MatchType,
    MatchWithParticipants, NewMatch,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden Definitions ====================

pub enum MatchIden {
    Table,
    Id,
    TournamentCategoryId,
    Participant1TeamId,
    Participant1PlayerId,
    Participant1PartnerId,
    Participant2TeamId,
    Participant2PlayerId,
    Participant2PartnerId,
    MatchType,
    MatchStatus,
    RoundNumber,
    MatchNumber,
    ScheduledDate,
    ActualStartDate,
    ActualEndDate,
    Venue,
    CourtNumber,
    WinnerParticipant,
    IsDraw,
    RefereeName,
    UmpireName,
    Notes,
    Metadata,
    CreatedAt,
    UpdatedAt,
}

impl Iden for MatchIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                MatchIden::Table => "matches",
                MatchIden::Id => "id",
                MatchIden::TournamentCategoryId => "tournament_category_id",
                MatchIden::Participant1TeamId => "participant1_team_id",
                MatchIden::Participant1PlayerId => "participant1_player_id",
                MatchIden::Participant1PartnerId => "participant1_partner_id",
                MatchIden::Participant2TeamId => "participant2_team_id",
                MatchIden::Participant2PlayerId => "participant2_player_id",
                MatchIden::Participant2PartnerId => "participant2_partner_id",
                MatchIden::MatchType => "match_type",
                MatchIden::MatchStatus => "match_status",
                MatchIden::RoundNumber => "round_number",
                MatchIden::MatchNumber => "match_number",
                MatchIden::ScheduledDate => "scheduled_date",
                MatchIden::ActualStartDate => "actual_start_date",
                MatchIden::ActualEndDate => "actual_end_date",
                MatchIden::Venue => "venue",
                MatchIden::CourtNumber => "court_number",
                MatchIden::WinnerParticipant => "winner_participant",
                MatchIden::IsDraw => "is_draw",
                MatchIden::RefereeName => "referee_name",
                MatchIden::UmpireName => "umpire_name",
                MatchIden::Notes => "notes",
                MatchIden::Metadata => "metadata",
                MatchIden::CreatedAt => "created_at",
                MatchIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

// ==================== Database Enum Types ====================

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "match_status", rename_all = "snake_case")]
pub enum MatchStatusDb {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Postponed,
    Forfeited,
    Bye,
}

impl From<MatchStatusDb> for MatchStatus {
    fn from(db: MatchStatusDb) -> Self {
        match db {
            MatchStatusDb::Scheduled => MatchStatus::Scheduled,
            MatchStatusDb::InProgress => MatchStatus::InProgress,
            MatchStatusDb::Completed => MatchStatus::Completed,
            MatchStatusDb::Cancelled => MatchStatus::Cancelled,
            MatchStatusDb::Postponed => MatchStatus::Postponed,
            MatchStatusDb::Forfeited => MatchStatus::Forfeited,
            MatchStatusDb::Bye => MatchStatus::Bye,
        }
    }
}

impl From<MatchStatus> for MatchStatusDb {
    fn from(status: MatchStatus) -> Self {
        match status {
            MatchStatus::Scheduled => MatchStatusDb::Scheduled,
            MatchStatus::InProgress => MatchStatusDb::InProgress,
            MatchStatus::Completed => MatchStatusDb::Completed,
            MatchStatus::Cancelled => MatchStatusDb::Cancelled,
            MatchStatus::Postponed => MatchStatusDb::Postponed,
            MatchStatus::Forfeited => MatchStatusDb::Forfeited,
            MatchStatus::Bye => MatchStatusDb::Bye,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "match_type", rename_all = "snake_case")]
pub enum MatchTypeDb {
    GroupStage,
    RoundOf128,
    RoundOf64,
    RoundOf32,
    RoundOf16,
    QuarterFinal,
    SemiFinal,
    ThirdPlace,
    Final,
    Qualifying,
    Playoff,
}

impl From<MatchTypeDb> for MatchType {
    fn from(db: MatchTypeDb) -> Self {
        match db {
            MatchTypeDb::GroupStage => MatchType::GroupStage,
            MatchTypeDb::RoundOf128 => MatchType::RoundOf128,
            MatchTypeDb::RoundOf64 => MatchType::RoundOf64,
            MatchTypeDb::RoundOf32 => MatchType::RoundOf32,
            MatchTypeDb::RoundOf16 => MatchType::RoundOf16,
            MatchTypeDb::QuarterFinal => MatchType::QuarterFinal,
            MatchTypeDb::SemiFinal => MatchType::SemiFinal,
            MatchTypeDb::ThirdPlace => MatchType::ThirdPlace,
            MatchTypeDb::Final => MatchType::Final,
            MatchTypeDb::Qualifying => MatchType::Qualifying,
            MatchTypeDb::Playoff => MatchType::Playoff,
        }
    }
}

impl From<MatchType> for MatchTypeDb {
    fn from(mt: MatchType) -> Self {
        match mt {
            MatchType::GroupStage => MatchTypeDb::GroupStage,
            MatchType::RoundOf128 => MatchTypeDb::RoundOf128,
            MatchType::RoundOf64 => MatchTypeDb::RoundOf64,
            MatchType::RoundOf32 => MatchTypeDb::RoundOf32,
            MatchType::RoundOf16 => MatchTypeDb::RoundOf16,
            MatchType::QuarterFinal => MatchTypeDb::QuarterFinal,
            MatchType::SemiFinal => MatchTypeDb::SemiFinal,
            MatchType::ThirdPlace => MatchTypeDb::ThirdPlace,
            MatchType::Final => MatchTypeDb::Final,
            MatchType::Qualifying => MatchTypeDb::Qualifying,
            MatchType::Playoff => MatchTypeDb::Playoff,
        }
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct MatchRow {
    id: Uuid,
    tournament_category_id: Uuid,
    participant1_team_id: Option<Uuid>,
    participant1_player_id: Option<Uuid>,
    participant1_partner_id: Option<Uuid>,
    participant2_team_id: Option<Uuid>,
    participant2_player_id: Option<Uuid>,
    participant2_partner_id: Option<Uuid>,
    match_type: MatchTypeDb,
    match_status: MatchStatusDb,
    round_number: Option<i32>,
    match_number: Option<i32>,
    scheduled_date: chrono::DateTime<Utc>,
    actual_start_date: Option<chrono::DateTime<Utc>>,
    actual_end_date: Option<chrono::DateTime<Utc>>,
    venue: Option<String>,
    court_number: Option<String>,
    winner_participant: Option<i32>,
    is_draw: bool,
    referee_name: Option<String>,
    umpire_name: Option<String>,
    notes: Option<String>,
    metadata: Option<JsonValue>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<MatchRow> for Match {
    fn from(row: MatchRow) -> Self {
        Match {
            id: row.id,
            tournament_category_id: row.tournament_category_id,
            participant1_team_id: row.participant1_team_id,
            participant1_player_id: row.participant1_player_id,
            participant1_partner_id: row.participant1_partner_id,
            participant2_team_id: row.participant2_team_id,
            participant2_player_id: row.participant2_player_id,
            participant2_partner_id: row.participant2_partner_id,
            match_type: row.match_type.into(),
            match_status: row.match_status.into(),
            round_number: row.round_number,
            match_number: row.match_number,
            scheduled_date: row.scheduled_date,
            actual_start_date: row.actual_start_date,
            actual_end_date: row.actual_end_date,
            venue: row.venue,
            court_number: row.court_number,
            winner_participant: row.winner_participant,
            is_draw: row.is_draw,
            referee_name: row.referee_name,
            umpire_name: row.umpire_name,
            notes: row.notes,
            metadata: row.metadata,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct MatchWithParticipantsRow {
    id: Uuid,
    tournament_category_id: Uuid,
    participant1_name: String,
    participant2_name: String,
    match_type: MatchTypeDb,
    match_status: MatchStatusDb,
    scheduled_date: chrono::DateTime<Utc>,
    venue: Option<String>,
    court_number: Option<String>,
    winner_participant: Option<i32>,
}

impl From<MatchWithParticipantsRow> for MatchWithParticipants {
    fn from(row: MatchWithParticipantsRow) -> Self {
        MatchWithParticipants {
            id: row.id,
            tournament_category_id: row.tournament_category_id,
            participant1_name: row.participant1_name,
            participant2_name: row.participant2_name,
            match_type: row.match_type.into(),
            match_status: row.match_status.into(),
            scheduled_date: row.scheduled_date,
            venue: row.venue,
            court_number: row.court_number,
            winner_participant: row.winner_participant,
        }
    }
}

#[derive(Debug, FromRow)]
struct MatchScheduleItemRow {
    id: Uuid,
    tournament_category_id: Uuid,
    tournament_name: String,
    category_name: String,
    participant1_name: String,
    participant2_name: String,
    match_type: MatchTypeDb,
    match_status: MatchStatusDb,
    scheduled_date: chrono::DateTime<Utc>,
    venue: Option<String>,
    court_number: Option<String>,
    round_number: Option<i32>,
}

impl From<MatchScheduleItemRow> for MatchScheduleItem {
    fn from(row: MatchScheduleItemRow) -> Self {
        MatchScheduleItem {
            id: row.id,
            tournament_category_id: row.tournament_category_id,
            tournament_name: row.tournament_name,
            category_name: row.category_name,
            participant1_name: row.participant1_name,
            participant2_name: row.participant2_name,
            match_type: row.match_type.into(),
            match_status: row.match_status.into(),
            scheduled_date: row.scheduled_date,
            venue: row.venue,
            court_number: row.court_number,
            round_number: row.round_number,
        }
    }
}

#[derive(Debug, FromRow)]
struct MatchAnalyticsRow {
    match_id: Uuid,
    total_duration_minutes: Option<i32>,
    sets_played: Option<i32>,
    participant1_score: Option<JsonValue>,
    participant2_score: Option<JsonValue>,
    rally_stats: Option<JsonValue>,
    performance_metrics: Option<JsonValue>,
}

impl From<MatchAnalyticsRow> for MatchAnalytics {
    fn from(row: MatchAnalyticsRow) -> Self {
        MatchAnalytics {
            match_id: row.match_id,
            total_duration_minutes: row.total_duration_minutes,
            sets_played: row.sets_played,
            participant1_score: row.participant1_score,
            participant2_score: row.participant2_score,
            rally_stats: row.rally_stats,
            performance_metrics: row.performance_metrics,
        }
    }
}

#[derive(Debug, FromRow)]
struct MatchStatisticsRow {
    match_id: Uuid,
    statistics: JsonValue,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<MatchStatisticsRow> for MatchStatistics {
    fn from(row: MatchStatisticsRow) -> Self {
        MatchStatistics {
            match_id: row.match_id,
            statistics: row.statistics,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ==================== Match Repository Implementation ====================

pub struct PgMatchRepository {
    pool: DbPool,
}

impl PgMatchRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn match_status_to_string(status: MatchStatus) -> String {
        match status {
            MatchStatus::Scheduled => "scheduled",
            MatchStatus::InProgress => "in_progress",
            MatchStatus::Completed => "completed",
            MatchStatus::Cancelled => "cancelled",
            MatchStatus::Postponed => "postponed",
            MatchStatus::Forfeited => "forfeited",
            MatchStatus::Bye => "bye",
        }
        .to_string()
    }

    fn match_type_to_string(mt: MatchType) -> String {
        match mt {
            MatchType::GroupStage => "group_stage",
            MatchType::RoundOf128 => "round_of128",
            MatchType::RoundOf64 => "round_of64",
            MatchType::RoundOf32 => "round_of32",
            MatchType::RoundOf16 => "round_of16",
            MatchType::QuarterFinal => "quarter_final",
            MatchType::SemiFinal => "semi_final",
            MatchType::ThirdPlace => "third_place",
            MatchType::Final => "final",
            MatchType::Qualifying => "qualifying",
            MatchType::Playoff => "playoff",
        }
        .to_string()
    }
}

#[async_trait]
impl MatchRepository for PgMatchRepository {
    async fn create(&self, new_match: NewMatch) -> Result<Match, AppError> {
        let id = Uuid::new_v4();

        let (sql, values) = Query::insert()
            .into_table(MatchIden::Table)
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
            ])
            .values_panic([
                id.into(),
                new_match.tournament_category_id.into(),
                new_match.participant1_team_id.into(),
                new_match.participant1_player_id.into(),
                new_match.participant1_partner_id.into(),
                new_match.participant2_team_id.into(),
                new_match.participant2_player_id.into(),
                new_match.participant2_partner_id.into(),
                Self::match_type_to_string(new_match.match_type).into(),
                "scheduled".into(),
                new_match.round_number.into(),
                new_match.match_number.into(),
                new_match.scheduled_date.into(),
                new_match.venue.into(),
                new_match.court_number.into(),
                new_match.referee_name.into(),
                new_match.umpire_name.into(),
                new_match.notes.into(),
                new_match.metadata.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: MatchRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(Match::from(row))
    }

    async fn find_by_id(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::ActualStartDate,
                MatchIden::ActualEndDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::WinnerParticipant,
                MatchIden::IsDraw,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
                MatchIden::CreatedAt,
                MatchIden::UpdatedAt,
            ])
            .from(MatchIden::Table)
            .and_where(Expr::col(MatchIden::Id).eq(match_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn update(
        &self,
        match_id: Uuid,
        match_data: EditableMatch,
    ) -> Result<Option<Match>, AppError> {
        let mut query = Query::update();
        query.table(MatchIden::Table);

        if let Some(status) = match_data.match_status {
            query.value(MatchIden::MatchStatus, Self::match_status_to_string(status));
        }
        if let Some(scheduled_date) = match_data.scheduled_date {
            query.value(MatchIden::ScheduledDate, scheduled_date);
        }
        if let Some(venue) = match_data.venue {
            query.value(MatchIden::Venue, venue);
        }
        if let Some(court_number) = match_data.court_number {
            query.value(MatchIden::CourtNumber, court_number);
        }
        if let Some(referee_name) = match_data.referee_name {
            query.value(MatchIden::RefereeName, referee_name);
        }
        if let Some(umpire_name) = match_data.umpire_name {
            query.value(MatchIden::UmpireName, umpire_name);
        }
        if let Some(notes) = match_data.notes {
            query.value(MatchIden::Notes, notes);
        }
        if let Some(metadata) = match_data.metadata {
            query.value(MatchIden::Metadata, metadata);
        }

        query.value(MatchIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(MatchIden::Id).eq(match_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn delete(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(MatchIden::Table)
            .and_where(Expr::col(MatchIden::Id).eq(match_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn find_by_tournament(&self, tournament_id: Uuid) -> Result<Vec<Match>, AppError> {
        let sql = r#"
            SELECT 
                m.id, m.tournament_category_id,
                m.participant1_team_id, m.participant1_player_id, m.participant1_partner_id,
                m.participant2_team_id, m.participant2_player_id, m.participant2_partner_id,
                m.match_type, m.match_status,
                m.round_number, m.match_number, m.scheduled_date, m.actual_start_date, m.actual_end_date,
                m.venue, m.court_number, m.winner_participant, m.is_draw,
                m.referee_name, m.umpire_name, m.notes, m.metadata, m.created_at, m.updated_at
            FROM matches m
            JOIN tournament_categories tc ON m.tournament_category_id = tc.id
            WHERE tc.tournament_id = $1
            ORDER BY m.scheduled_date, m.round_number, m.match_number
        "#;

        let rows: Vec<MatchRow> = sqlx::query_as(sql)
            .bind(tournament_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Match::from).collect())
    }

    async fn find_by_category(&self, category_id: Uuid) -> Result<Vec<Match>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::ActualStartDate,
                MatchIden::ActualEndDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::WinnerParticipant,
                MatchIden::IsDraw,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
                MatchIden::CreatedAt,
                MatchIden::UpdatedAt,
            ])
            .from(MatchIden::Table)
            .and_where(Expr::col(MatchIden::TournamentCategoryId).eq(category_id))
            .order_by(MatchIden::RoundNumber, sea_query::Order::Asc)
            .order_by(MatchIden::MatchNumber, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Match::from).collect())
    }

    async fn find_scheduled(&self) -> Result<Vec<MatchScheduleItem>, AppError> {
        let sql = r#"
            SELECT 
                m.id, m.tournament_category_id,
                t.name as tournament_name, tc.name as category_name,
                COALESCE(t1.name, CONCAT(p1.name, COALESCE(CONCAT(' / ', pp1.name), '')), 'TBD') as participant1_name,
                COALESCE(t2.name, CONCAT(p2.name, COALESCE(CONCAT(' / ', pp2.name), '')), 'TBD') as participant2_name,
                m.match_type, m.match_status, m.scheduled_date, m.venue, m.court_number, m.round_number
            FROM matches m
            JOIN tournament_categories tc ON m.tournament_category_id = tc.id
            JOIN tournaments t ON tc.tournament_id = t.id
            LEFT JOIN teams t1 ON m.participant1_team_id = t1.id
            LEFT JOIN teams t2 ON m.participant2_team_id = t2.id
            LEFT JOIN players p1 ON m.participant1_player_id = p1.id
            LEFT JOIN players p2 ON m.participant2_player_id = p2.id
            LEFT JOIN players pp1 ON m.participant1_partner_id = pp1.id
            LEFT JOIN players pp2 ON m.participant2_partner_id = pp2.id
            WHERE m.match_status = 'scheduled' AND m.scheduled_date IS NOT NULL
            ORDER BY m.scheduled_date ASC
        "#;

        let rows: Vec<MatchScheduleItemRow> = sqlx::query_as(sql).fetch_all(&self.pool).await?;

        Ok(rows.into_iter().map(MatchScheduleItem::from).collect())
    }

    async fn find_with_participants(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchWithParticipants>, AppError> {
        let sql = r#"
            SELECT 
                m.id, m.tournament_category_id,
                COALESCE(
                    CASE 
                        WHEN m.participant1_team_id IS NOT NULL THEN t1.name
                        ELSE CONCAT(p1.name, 
                                   CASE WHEN m.participant1_partner_id IS NOT NULL 
                                        THEN CONCAT(' / ', p1p.name) 
                                        ELSE '' END)
                    END, 'TBD'
                ) as participant1_name,
                COALESCE(
                    CASE 
                        WHEN m.participant2_team_id IS NOT NULL THEN t2.name
                        ELSE CONCAT(p2.name,
                                   CASE WHEN m.participant2_partner_id IS NOT NULL 
                                        THEN CONCAT(' / ', p2p.name) 
                                        ELSE '' END)
                    END, 'TBD'
                ) as participant2_name,
                m.match_type, m.match_status, m.scheduled_date, m.venue, m.court_number, m.winner_participant
            FROM matches m
            LEFT JOIN teams t1 ON m.participant1_team_id = t1.id
            LEFT JOIN teams t2 ON m.participant2_team_id = t2.id
            LEFT JOIN players p1 ON m.participant1_player_id = p1.id
            LEFT JOIN players p2 ON m.participant2_player_id = p2.id
            LEFT JOIN players p1p ON m.participant1_partner_id = p1p.id
            LEFT JOIN players p2p ON m.participant2_partner_id = p2p.id
            WHERE m.id = $1
        "#;

        let row: Option<MatchWithParticipantsRow> = sqlx::query_as(sql)
            .bind(match_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchWithParticipants::from))
    }

    async fn update_status(
        &self,
        match_id: Uuid,
        status: MatchStatus,
    ) -> Result<Option<Match>, AppError> {
        let mut query = Query::update();
        query.table(MatchIden::Table);
        query.value(MatchIden::MatchStatus, Self::match_status_to_string(status));

        // Set actual_start_date when match starts
        if matches!(status, MatchStatus::InProgress) {
            query.value(MatchIden::ActualStartDate, Utc::now());
        }

        // Set actual_end_date when match ends
        if matches!(
            status,
            MatchStatus::Completed | MatchStatus::Cancelled | MatchStatus::Forfeited
        ) {
            query.value(MatchIden::ActualEndDate, Utc::now());
        }

        query.value(MatchIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(MatchIden::Id).eq(match_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn start_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        let (sql, values) = Query::update()
            .table(MatchIden::Table)
            .value(MatchIden::MatchStatus, "in_progress")
            .value(MatchIden::ActualStartDate, Utc::now())
            .value(MatchIden::UpdatedAt, Utc::now())
            .and_where(Expr::col(MatchIden::Id).eq(match_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn complete_match(
        &self,
        match_id: Uuid,
        winner: i32,
        is_draw: bool,
    ) -> Result<Option<Match>, AppError> {
        let (sql, values) = Query::update()
            .table(MatchIden::Table)
            .value(MatchIden::MatchStatus, "completed")
            .value(MatchIden::ActualEndDate, Utc::now())
            .value(MatchIden::WinnerParticipant, winner)
            .value(MatchIden::IsDraw, is_draw)
            .value(MatchIden::UpdatedAt, Utc::now())
            .and_where(Expr::col(MatchIden::Id).eq(match_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn cancel_match(&self, match_id: Uuid, reason: &str) -> Result<Option<Match>, AppError> {
        let notes = format!("Cancelled: {}", reason);

        let (sql, values) = Query::update()
            .table(MatchIden::Table)
            .value(MatchIden::MatchStatus, "cancelled")
            .value(MatchIden::ActualEndDate, Utc::now())
            .value(MatchIden::Notes, notes)
            .value(MatchIden::UpdatedAt, Utc::now())
            .and_where(Expr::col(MatchIden::Id).eq(match_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn postpone_match(&self, match_id: Uuid) -> Result<Option<Match>, AppError> {
        let (sql, values) = Query::update()
            .table(MatchIden::Table)
            .value(MatchIden::MatchStatus, "postponed")
            .value(MatchIden::UpdatedAt, Utc::now())
            .and_where(Expr::col(MatchIden::Id).eq(match_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn find_user_upcoming_matches(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MatchScheduleItem>, AppError> {
        let sql = r#"
            SELECT DISTINCT
                m.id, m.tournament_category_id,
                t.name as tournament_name, tc.name as category_name,
                COALESCE(t1.name, CONCAT(p1.name, COALESCE(CONCAT(' / ', pp1.name), '')), 'TBD') as participant1_name,
                COALESCE(t2.name, CONCAT(p2.name, COALESCE(CONCAT(' / ', pp2.name), '')), 'TBD') as participant2_name,
                m.match_type, m.match_status, m.scheduled_date, m.venue, m.court_number, m.round_number
            FROM matches m
            JOIN tournament_categories tc ON m.tournament_category_id = tc.id
            JOIN tournaments t ON tc.tournament_id = t.id
            LEFT JOIN teams t1 ON m.participant1_team_id = t1.id
            LEFT JOIN teams t2 ON m.participant2_team_id = t2.id
            LEFT JOIN team_members tm1 ON t1.id = tm1.team_id
            LEFT JOIN team_members tm2 ON t2.id = tm2.team_id
            LEFT JOIN players p1 ON m.participant1_player_id = p1.id
            LEFT JOIN players p2 ON m.participant2_player_id = p2.id
            LEFT JOIN players pp1 ON m.participant1_partner_id = pp1.id
            LEFT JOIN players pp2 ON m.participant2_partner_id = pp2.id
            WHERE (
                p1.user_id = $1 OR pp1.user_id = $1 OR
                p2.user_id = $1 OR pp2.user_id = $1 OR
                EXISTS (SELECT 1 FROM players p WHERE p.user_id = $1 AND (tm1.player_id = p.id OR tm2.player_id = p.id))
            )
            AND m.match_status IN ('scheduled', 'in_progress')
            AND m.scheduled_date >= NOW()
            ORDER BY m.scheduled_date ASC
        "#;

        let rows: Vec<MatchScheduleItemRow> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(MatchScheduleItem::from).collect())
    }

    async fn find_user_match_history(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MatchScheduleItem>, AppError> {
        let sql = r#"
            SELECT DISTINCT
                m.id, m.tournament_category_id,
                t.name as tournament_name, tc.name as category_name,
                COALESCE(t1.name, CONCAT(p1.name, COALESCE(CONCAT(' / ', pp1.name), '')), 'TBD') as participant1_name,
                COALESCE(t2.name, CONCAT(p2.name, COALESCE(CONCAT(' / ', pp2.name), '')), 'TBD') as participant2_name,
                m.match_type, m.match_status, m.scheduled_date, m.venue, m.court_number, m.round_number
            FROM matches m
            JOIN tournament_categories tc ON m.tournament_category_id = tc.id
            JOIN tournaments t ON tc.tournament_id = t.id
            LEFT JOIN teams t1 ON m.participant1_team_id = t1.id
            LEFT JOIN teams t2 ON m.participant2_team_id = t2.id
            LEFT JOIN team_members tm1 ON t1.id = tm1.team_id
            LEFT JOIN team_members tm2 ON t2.id = tm2.team_id
            LEFT JOIN players p1 ON m.participant1_player_id = p1.id
            LEFT JOIN players p2 ON m.participant2_player_id = p2.id
            LEFT JOIN players pp1 ON m.participant1_partner_id = pp1.id
            LEFT JOIN players pp2 ON m.participant2_partner_id = pp2.id
            WHERE (
                p1.user_id = $1 OR pp1.user_id = $1 OR
                p2.user_id = $1 OR pp2.user_id = $1 OR
                EXISTS (SELECT 1 FROM players p WHERE p.user_id = $1 AND (tm1.player_id = p.id OR tm2.player_id = p.id))
            )
            AND m.match_status IN ('completed', 'cancelled', 'forfeited')
            ORDER BY m.actual_end_date DESC, m.scheduled_date DESC
        "#;

        let rows: Vec<MatchScheduleItemRow> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(MatchScheduleItem::from).collect())
    }

    async fn find_live_matches(&self) -> Result<Vec<Match>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::ActualStartDate,
                MatchIden::ActualEndDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::WinnerParticipant,
                MatchIden::IsDraw,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
                MatchIden::CreatedAt,
                MatchIden::UpdatedAt,
            ])
            .from(MatchIden::Table)
            .and_where(Expr::col(MatchIden::MatchStatus).eq("in_progress"))
            .order_by(MatchIden::ActualStartDate, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Match::from).collect())
    }

    async fn update_live_match(
        &self,
        match_id: Uuid,
        update: LiveMatchUpdate,
    ) -> Result<Option<Match>, AppError> {
        let metadata = serde_json::json!({
            "live_update": {
                "current_score": update.current_score,
                "game_time": update.game_time,
                "current_set": update.current_set,
                "timestamp": Utc::now()
            },
            "existing": update.metadata
        });

        let mut query = Query::update();
        query.table(MatchIden::Table);
        query.value(MatchIden::Metadata, metadata);

        if let Some(notes) = update.notes {
            query.value(MatchIden::Notes, notes);
        }

        query.value(MatchIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(MatchIden::Id).eq(match_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Match::from))
    }

    async fn get_match_analytics(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchAnalytics>, AppError> {
        let sql = r#"
            SELECT 
                m.id as match_id,
                EXTRACT(EPOCH FROM (m.actual_end_date - m.actual_start_date))::int / 60 as total_duration_minutes,
                1 as sets_played,
                '{}'::jsonb as participant1_score,
                '{}'::jsonb as participant2_score,
                '{}'::jsonb as rally_stats,
                '{}'::jsonb as performance_metrics
            FROM matches m
            WHERE m.id = $1 AND m.match_status = 'completed'
        "#;

        let row: Option<MatchAnalyticsRow> = sqlx::query_as(sql)
            .bind(match_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchAnalytics::from))
    }

    async fn get_match_statistics(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchStatistics>, AppError> {
        let sql = r#"
            SELECT 
                $1::uuid as match_id,
                '{"placeholder": "statistics"}'::jsonb as statistics,
                NOW() as created_at,
                NOW() as updated_at
        "#;

        let row: Option<MatchStatisticsRow> = sqlx::query_as(sql)
            .bind(match_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchStatistics::from))
    }

    async fn get_match_media(&self, _match_id: Uuid) -> Result<Vec<MatchMedia>, AppError> {
        // TODO: Placeholder - would need actual match_media table
        Ok(vec![])
    }

    async fn upload_match_media(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        media_type: &str,
        file_url: &str,
    ) -> Result<MatchMedia, AppError> {
        // TODO: Placeholder implementation
        let media = MatchMedia {
            id: Uuid::new_v4(),
            match_id,
            media_type: media_type.to_string(),
            file_url: file_url.to_string(),
            thumbnail_url: None,
            file_size: None,
            duration: None,
            uploaded_by: user_id,
            created_at: Utc::now(),
        };

        Ok(media)
    }

    async fn get_match_comments(&self, _match_id: Uuid) -> Result<Vec<MatchComment>, AppError> {
        // TODO: Placeholder - would need actual match_comments table
        Ok(vec![])
    }

    async fn add_match_comment(
        &self,
        match_id: Uuid,
        user_id: Uuid,
        comment: &str,
    ) -> Result<MatchComment, AppError> {
        // TODO: Placeholder implementation
        let match_comment = MatchComment {
            id: Uuid::new_v4(),
            match_id,
            user_id,
            comment: comment.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(match_comment)
    }

    async fn subscribe_to_match(
        &self,
        match_id: Uuid,
        user_id: Uuid,
    ) -> Result<MatchSubscription, AppError> {
        // TODO: Placeholder implementation
        let subscription = MatchSubscription {
            id: Uuid::new_v4(),
            match_id,
            user_id,
            notification_preferences: serde_json::json!({"all": true}),
            created_at: Utc::now(),
        };

        Ok(subscription)
    }

    async fn unsubscribe_from_match(
        &self,
        _match_id: Uuid,
        _user_id: Uuid,
    ) -> Result<(), AppError> {
        // TODO: Placeholder implementation
        Ok(())
    }

    async fn bulk_update_matches(
        &self,
        match_ids: Vec<Uuid>,
        updates: EditableMatch,
    ) -> Result<Vec<Match>, AppError> {
        let mut updated_matches = Vec::new();

        for match_id in match_ids {
            if let Some(updated_match) = self.update(match_id, updates.clone()).await? {
                updated_matches.push(updated_match);
            }
        }

        Ok(updated_matches)
    }

    async fn bulk_cancel_matches(
        &self,
        match_ids: Vec<Uuid>,
        reason: &str,
    ) -> Result<Vec<Match>, AppError> {
        let mut cancelled_matches = Vec::new();

        for match_id in match_ids {
            if let Some(cancelled_match) = self.cancel_match(match_id, reason).await? {
                cancelled_matches.push(cancelled_match);
            }
        }

        Ok(cancelled_matches)
    }
}
