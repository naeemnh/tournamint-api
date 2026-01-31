use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::tournament::{
    BracketStatus, BracketType, EditableTournament, NewTournament, PaymentStatus,
    RegistrationStatus, SportType, TeamComposition, Tournament, TournamentFormat,
    TournamentRepository, TournamentStatus,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden Definitions ====================
// These are shared across tournament-related repositories

pub enum TournamentIden {
    Table,
    Id,
    Name,
    Description,
    SportType,
    Format,
    Status,
    StartDate,
    EndDate,
    RegistrationStartDate,
    RegistrationEndDate,
    Venue,
    MaxParticipants,
    EntryFee,
    PrizePool,
    Rules,
    OrganizerId,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentIden::Table => "tournaments",
                TournamentIden::Id => "id",
                TournamentIden::Name => "name",
                TournamentIden::Description => "description",
                TournamentIden::SportType => "sport_type",
                TournamentIden::Format => "format",
                TournamentIden::Status => "status",
                TournamentIden::StartDate => "start_date",
                TournamentIden::EndDate => "end_date",
                TournamentIden::RegistrationStartDate => "registration_start_date",
                TournamentIden::RegistrationEndDate => "registration_end_date",
                TournamentIden::Venue => "venue",
                TournamentIden::MaxParticipants => "max_participants",
                TournamentIden::EntryFee => "entry_fee",
                TournamentIden::PrizePool => "prize_pool",
                TournamentIden::Rules => "rules",
                TournamentIden::OrganizerId => "organizer_id",
                TournamentIden::CreatedAt => "created_at",
                TournamentIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

// ==================== Database Enum Types ====================
// Shared enum mappings for tournament-related tables

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "sport_type", rename_all = "snake_case")]
pub enum SportTypeDb {
    Basketball,
    TableTennis,
    Volleyball,
    Badminton,
    Tennis,
    Football,
    Cricket,
    Chess,
    Esports,
}

impl From<SportTypeDb> for SportType {
    fn from(db: SportTypeDb) -> Self {
        match db {
            SportTypeDb::Basketball => SportType::Basketball,
            SportTypeDb::TableTennis => SportType::TableTennis,
            SportTypeDb::Volleyball => SportType::Volleyball,
            SportTypeDb::Badminton => SportType::Badminton,
            SportTypeDb::Tennis => SportType::Tennis,
            SportTypeDb::Football => SportType::Football,
            SportTypeDb::Cricket => SportType::Cricket,
            SportTypeDb::Chess => SportType::Chess,
            SportTypeDb::Esports => SportType::Esports,
        }
    }
}

impl From<SportType> for SportTypeDb {
    fn from(s: SportType) -> Self {
        match s {
            SportType::Basketball => SportTypeDb::Basketball,
            SportType::TableTennis => SportTypeDb::TableTennis,
            SportType::Volleyball => SportTypeDb::Volleyball,
            SportType::Badminton => SportTypeDb::Badminton,
            SportType::Tennis => SportTypeDb::Tennis,
            SportType::Football => SportTypeDb::Football,
            SportType::Cricket => SportTypeDb::Cricket,
            SportType::Chess => SportTypeDb::Chess,
            SportType::Esports => SportTypeDb::Esports,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "tournament_format", rename_all = "snake_case")]
pub enum TournamentFormatDb {
    Elimination,
    DoubleElimination,
    RoundRobin,
    League,
    Swiss,
    GroupsAndKnockout,
}

impl From<TournamentFormatDb> for TournamentFormat {
    fn from(db: TournamentFormatDb) -> Self {
        match db {
            TournamentFormatDb::Elimination => TournamentFormat::Elimination,
            TournamentFormatDb::DoubleElimination => TournamentFormat::DoubleElimination,
            TournamentFormatDb::RoundRobin => TournamentFormat::RoundRobin,
            TournamentFormatDb::League => TournamentFormat::League,
            TournamentFormatDb::Swiss => TournamentFormat::Swiss,
            TournamentFormatDb::GroupsAndKnockout => TournamentFormat::GroupsAndKnockout,
        }
    }
}

impl From<TournamentFormat> for TournamentFormatDb {
    fn from(f: TournamentFormat) -> Self {
        match f {
            TournamentFormat::Elimination => TournamentFormatDb::Elimination,
            TournamentFormat::DoubleElimination => TournamentFormatDb::DoubleElimination,
            TournamentFormat::RoundRobin => TournamentFormatDb::RoundRobin,
            TournamentFormat::League => TournamentFormatDb::League,
            TournamentFormat::Swiss => TournamentFormatDb::Swiss,
            TournamentFormat::GroupsAndKnockout => TournamentFormatDb::GroupsAndKnockout,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "tournament_status", rename_all = "snake_case")]
pub enum TournamentStatusDb {
    Draft,
    Upcoming,
    RegistrationOpen,
    RegistrationClosed,
    InProgress,
    Completed,
    Cancelled,
}

impl From<TournamentStatusDb> for TournamentStatus {
    fn from(db: TournamentStatusDb) -> Self {
        match db {
            TournamentStatusDb::Draft => TournamentStatus::Draft,
            TournamentStatusDb::Upcoming => TournamentStatus::Upcoming,
            TournamentStatusDb::RegistrationOpen => TournamentStatus::RegistrationOpen,
            TournamentStatusDb::RegistrationClosed => TournamentStatus::RegistrationClosed,
            TournamentStatusDb::InProgress => TournamentStatus::InProgress,
            TournamentStatusDb::Completed => TournamentStatus::Completed,
            TournamentStatusDb::Cancelled => TournamentStatus::Cancelled,
        }
    }
}

impl From<TournamentStatus> for TournamentStatusDb {
    fn from(s: TournamentStatus) -> Self {
        match s {
            TournamentStatus::Draft => TournamentStatusDb::Draft,
            TournamentStatus::Upcoming => TournamentStatusDb::Upcoming,
            TournamentStatus::RegistrationOpen => TournamentStatusDb::RegistrationOpen,
            TournamentStatus::RegistrationClosed => TournamentStatusDb::RegistrationClosed,
            TournamentStatus::InProgress => TournamentStatusDb::InProgress,
            TournamentStatus::Completed => TournamentStatusDb::Completed,
            TournamentStatus::Cancelled => TournamentStatusDb::Cancelled,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "team_composition", rename_all = "snake_case")]
pub enum TeamCompositionDb {
    Singles,
    Doubles,
    MixedDoubles,
    Team,
}

impl From<TeamCompositionDb> for TeamComposition {
    fn from(db: TeamCompositionDb) -> Self {
        match db {
            TeamCompositionDb::Singles => TeamComposition::Singles,
            TeamCompositionDb::Doubles => TeamComposition::Doubles,
            TeamCompositionDb::MixedDoubles => TeamComposition::MixedDoubles,
            TeamCompositionDb::Team => TeamComposition::Team,
        }
    }
}

impl From<TeamComposition> for TeamCompositionDb {
    fn from(t: TeamComposition) -> Self {
        match t {
            TeamComposition::Singles => TeamCompositionDb::Singles,
            TeamComposition::Doubles => TeamCompositionDb::Doubles,
            TeamComposition::MixedDoubles => TeamCompositionDb::MixedDoubles,
            TeamComposition::Team => TeamCompositionDb::Team,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "registration_status", rename_all = "snake_case")]
pub enum RegistrationStatusDb {
    Pending,
    Approved,
    Rejected,
    Withdrawn,
    Waitlisted,
}

impl From<RegistrationStatusDb> for RegistrationStatus {
    fn from(db: RegistrationStatusDb) -> Self {
        match db {
            RegistrationStatusDb::Pending => RegistrationStatus::Pending,
            RegistrationStatusDb::Approved => RegistrationStatus::Approved,
            RegistrationStatusDb::Rejected => RegistrationStatus::Rejected,
            RegistrationStatusDb::Withdrawn => RegistrationStatus::Withdrawn,
            RegistrationStatusDb::Waitlisted => RegistrationStatus::Waitlisted,
        }
    }
}

impl From<RegistrationStatus> for RegistrationStatusDb {
    fn from(r: RegistrationStatus) -> Self {
        match r {
            RegistrationStatus::Pending => RegistrationStatusDb::Pending,
            RegistrationStatus::Approved => RegistrationStatusDb::Approved,
            RegistrationStatus::Rejected => RegistrationStatusDb::Rejected,
            RegistrationStatus::Withdrawn => RegistrationStatusDb::Withdrawn,
            RegistrationStatus::Waitlisted => RegistrationStatusDb::Waitlisted,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "snake_case")]
pub enum PaymentStatusDb {
    Pending,
    Completed,
    Failed,
    Refunded,
    Waived,
}

impl From<PaymentStatusDb> for PaymentStatus {
    fn from(db: PaymentStatusDb) -> Self {
        match db {
            PaymentStatusDb::Pending => PaymentStatus::Pending,
            PaymentStatusDb::Completed => PaymentStatus::Completed,
            PaymentStatusDb::Failed => PaymentStatus::Failed,
            PaymentStatusDb::Refunded => PaymentStatus::Refunded,
            PaymentStatusDb::Waived => PaymentStatus::Waived,
        }
    }
}

impl From<PaymentStatus> for PaymentStatusDb {
    fn from(p: PaymentStatus) -> Self {
        match p {
            PaymentStatus::Pending => PaymentStatusDb::Pending,
            PaymentStatus::Completed => PaymentStatusDb::Completed,
            PaymentStatus::Failed => PaymentStatusDb::Failed,
            PaymentStatus::Refunded => PaymentStatusDb::Refunded,
            PaymentStatus::Waived => PaymentStatusDb::Waived,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "bracket_type", rename_all = "snake_case")]
pub enum BracketTypeDb {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    Swiss,
    GroupStage,
}

impl From<BracketTypeDb> for BracketType {
    fn from(db: BracketTypeDb) -> Self {
        match db {
            BracketTypeDb::SingleElimination => BracketType::SingleElimination,
            BracketTypeDb::DoubleElimination => BracketType::DoubleElimination,
            BracketTypeDb::RoundRobin => BracketType::RoundRobin,
            BracketTypeDb::Swiss => BracketType::Swiss,
            BracketTypeDb::GroupStage => BracketType::GroupStage,
        }
    }
}

impl From<BracketType> for BracketTypeDb {
    fn from(b: BracketType) -> Self {
        match b {
            BracketType::SingleElimination => BracketTypeDb::SingleElimination,
            BracketType::DoubleElimination => BracketTypeDb::DoubleElimination,
            BracketType::RoundRobin => BracketTypeDb::RoundRobin,
            BracketType::Swiss => BracketTypeDb::Swiss,
            BracketType::GroupStage => BracketTypeDb::GroupStage,
        }
    }
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "bracket_status", rename_all = "snake_case")]
pub enum BracketStatusDb {
    NotGenerated,
    Generated,
    InProgress,
    Completed,
}

impl From<BracketStatusDb> for BracketStatus {
    fn from(db: BracketStatusDb) -> Self {
        match db {
            BracketStatusDb::NotGenerated => BracketStatus::NotGenerated,
            BracketStatusDb::Generated => BracketStatus::Generated,
            BracketStatusDb::InProgress => BracketStatus::InProgress,
            BracketStatusDb::Completed => BracketStatus::Completed,
        }
    }
}

impl From<BracketStatus> for BracketStatusDb {
    fn from(b: BracketStatus) -> Self {
        match b {
            BracketStatus::NotGenerated => BracketStatusDb::NotGenerated,
            BracketStatus::Generated => BracketStatusDb::Generated,
            BracketStatus::InProgress => BracketStatusDb::InProgress,
            BracketStatus::Completed => BracketStatusDb::Completed,
        }
    }
}

// ==================== Helper Functions ====================

pub fn sport_type_to_string(s: SportType) -> String {
    serde_json::to_string(&s).unwrap().trim_matches('"').to_string()
}

pub fn format_to_string(f: TournamentFormat) -> String {
    serde_json::to_string(&f).unwrap().trim_matches('"').to_string()
}

pub fn status_to_string(s: TournamentStatus) -> String {
    serde_json::to_string(&s).unwrap().trim_matches('"').to_string()
}

pub fn team_composition_to_string(t: TeamComposition) -> String {
    serde_json::to_string(&t).unwrap().trim_matches('"').to_string()
}

pub fn registration_status_to_string(r: RegistrationStatus) -> String {
    serde_json::to_string(&r).unwrap().trim_matches('"').to_string()
}

pub fn payment_status_to_string(p: PaymentStatus) -> String {
    serde_json::to_string(&p).unwrap().trim_matches('"').to_string()
}

pub fn bracket_type_to_string(b: BracketType) -> String {
    serde_json::to_string(&b).unwrap().trim_matches('"').to_string()
}

pub fn bracket_status_to_string(b: BracketStatus) -> String {
    serde_json::to_string(&b).unwrap().trim_matches('"').to_string()
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TournamentRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    sport_type: SportTypeDb,
    format: TournamentFormatDb,
    status: TournamentStatusDb,
    start_date: chrono::DateTime<Utc>,
    end_date: chrono::DateTime<Utc>,
    registration_start_date: Option<chrono::DateTime<Utc>>,
    registration_end_date: Option<chrono::DateTime<Utc>>,
    venue: Option<String>,
    max_participants: Option<i32>,
    entry_fee: Option<Decimal>,
    prize_pool: Option<Decimal>,
    rules: Option<JsonValue>,
    organizer_id: Uuid,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<TournamentRow> for Tournament {
    fn from(row: TournamentRow) -> Self {
        Tournament {
            id: row.id,
            name: row.name,
            description: row.description,
            sport_type: row.sport_type.into(),
            format: row.format.into(),
            status: row.status.into(),
            start_date: row.start_date,
            end_date: row.end_date,
            registration_start_date: row.registration_start_date,
            registration_end_date: row.registration_end_date,
            venue: row.venue,
            max_participants: row.max_participants,
            entry_fee: row.entry_fee,
            prize_pool: row.prize_pool,
            rules: row.rules,
            organizer_id: row.organizer_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ==================== Tournament Repository ====================

pub struct PgTournamentRepository {
    pool: DbPool,
}

impl PgTournamentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TournamentRepository for PgTournamentRepository {
    async fn create(&self, new_tournament: NewTournament) -> Result<Tournament, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TournamentIden::Table)
            .columns([
                TournamentIden::Name,
                TournamentIden::Description,
                TournamentIden::SportType,
                TournamentIden::Format,
                TournamentIden::StartDate,
                TournamentIden::EndDate,
                TournamentIden::RegistrationStartDate,
                TournamentIden::RegistrationEndDate,
                TournamentIden::Venue,
                TournamentIden::MaxParticipants,
                TournamentIden::EntryFee,
                TournamentIden::PrizePool,
                TournamentIden::Rules,
                TournamentIden::OrganizerId,
            ])
            .values_panic([
                new_tournament.name.into(),
                new_tournament.description.into(),
                sport_type_to_string(new_tournament.sport_type).into(),
                format_to_string(new_tournament.format).into(),
                new_tournament.start_date.into(),
                new_tournament.end_date.into(),
                new_tournament.registration_start_date.into(),
                new_tournament.registration_end_date.into(),
                new_tournament.venue.into(),
                new_tournament.max_participants.into(),
                new_tournament.entry_fee.into(),
                new_tournament.prize_pool.into(),
                new_tournament.rules.into(),
                new_tournament.organizer_id.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TournamentRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(Tournament::from(row))
    }

    async fn get_all(&self) -> Result<Vec<Tournament>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentIden::Id,
                TournamentIden::Name,
                TournamentIden::Description,
                TournamentIden::SportType,
                TournamentIden::Format,
                TournamentIden::Status,
                TournamentIden::StartDate,
                TournamentIden::EndDate,
                TournamentIden::RegistrationStartDate,
                TournamentIden::RegistrationEndDate,
                TournamentIden::Venue,
                TournamentIden::MaxParticipants,
                TournamentIden::EntryFee,
                TournamentIden::PrizePool,
                TournamentIden::Rules,
                TournamentIden::OrganizerId,
                TournamentIden::CreatedAt,
                TournamentIden::UpdatedAt,
            ])
            .from(TournamentIden::Table)
            .order_by(TournamentIden::StartDate, sea_query::Order::Desc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Tournament::from).collect())
    }

    async fn get_by_id(&self, tournament_id: Uuid) -> Result<Option<Tournament>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentIden::Id,
                TournamentIden::Name,
                TournamentIden::Description,
                TournamentIden::SportType,
                TournamentIden::Format,
                TournamentIden::Status,
                TournamentIden::StartDate,
                TournamentIden::EndDate,
                TournamentIden::RegistrationStartDate,
                TournamentIden::RegistrationEndDate,
                TournamentIden::Venue,
                TournamentIden::MaxParticipants,
                TournamentIden::EntryFee,
                TournamentIden::PrizePool,
                TournamentIden::Rules,
                TournamentIden::OrganizerId,
                TournamentIden::CreatedAt,
                TournamentIden::UpdatedAt,
            ])
            .from(TournamentIden::Table)
            .and_where(Expr::col(TournamentIden::Id).eq(tournament_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Tournament::from))
    }

    async fn get_by_status(&self, status: TournamentStatus) -> Result<Vec<Tournament>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentIden::Id,
                TournamentIden::Name,
                TournamentIden::Description,
                TournamentIden::SportType,
                TournamentIden::Format,
                TournamentIden::Status,
                TournamentIden::StartDate,
                TournamentIden::EndDate,
                TournamentIden::RegistrationStartDate,
                TournamentIden::RegistrationEndDate,
                TournamentIden::Venue,
                TournamentIden::MaxParticipants,
                TournamentIden::EntryFee,
                TournamentIden::PrizePool,
                TournamentIden::Rules,
                TournamentIden::OrganizerId,
                TournamentIden::CreatedAt,
                TournamentIden::UpdatedAt,
            ])
            .from(TournamentIden::Table)
            .and_where(Expr::col(TournamentIden::Status).eq(status_to_string(status)))
            .order_by(TournamentIden::StartDate, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Tournament::from).collect())
    }

    async fn get_by_organizer(&self, organizer_id: Uuid) -> Result<Vec<Tournament>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentIden::Id,
                TournamentIden::Name,
                TournamentIden::Description,
                TournamentIden::SportType,
                TournamentIden::Format,
                TournamentIden::Status,
                TournamentIden::StartDate,
                TournamentIden::EndDate,
                TournamentIden::RegistrationStartDate,
                TournamentIden::RegistrationEndDate,
                TournamentIden::Venue,
                TournamentIden::MaxParticipants,
                TournamentIden::EntryFee,
                TournamentIden::PrizePool,
                TournamentIden::Rules,
                TournamentIden::OrganizerId,
                TournamentIden::CreatedAt,
                TournamentIden::UpdatedAt,
            ])
            .from(TournamentIden::Table)
            .and_where(Expr::col(TournamentIden::OrganizerId).eq(organizer_id))
            .order_by(TournamentIden::StartDate, sea_query::Order::Desc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(Tournament::from).collect())
    }

    async fn update(
        &self,
        tournament_id: Uuid,
        tournament_data: EditableTournament,
    ) -> Result<Option<Tournament>, AppError> {
        let mut query = Query::update();
        query.table(TournamentIden::Table);

        if let Some(name) = tournament_data.name {
            query.value(TournamentIden::Name, name);
        }
        if let Some(description) = tournament_data.description {
            query.value(TournamentIden::Description, description);
        }
        if let Some(sport_type) = tournament_data.sport_type {
            query.value(TournamentIden::SportType, sport_type_to_string(sport_type));
        }
        if let Some(format) = tournament_data.format {
            query.value(TournamentIden::Format, format_to_string(format));
        }
        if let Some(status) = tournament_data.status {
            query.value(TournamentIden::Status, status_to_string(status));
        }
        if let Some(start_date) = tournament_data.start_date {
            query.value(TournamentIden::StartDate, start_date);
        }
        if let Some(end_date) = tournament_data.end_date {
            query.value(TournamentIden::EndDate, end_date);
        }
        if let Some(registration_start_date) = tournament_data.registration_start_date {
            query.value(TournamentIden::RegistrationStartDate, registration_start_date);
        }
        if let Some(registration_end_date) = tournament_data.registration_end_date {
            query.value(TournamentIden::RegistrationEndDate, registration_end_date);
        }
        if let Some(venue) = tournament_data.venue {
            query.value(TournamentIden::Venue, venue);
        }
        if let Some(max_participants) = tournament_data.max_participants {
            query.value(TournamentIden::MaxParticipants, max_participants);
        }
        if let Some(entry_fee) = tournament_data.entry_fee {
            query.value(TournamentIden::EntryFee, entry_fee);
        }
        if let Some(prize_pool) = tournament_data.prize_pool {
            query.value(TournamentIden::PrizePool, prize_pool);
        }
        if let Some(rules) = tournament_data.rules {
            query.value(TournamentIden::Rules, rules);
        }

        query.value(TournamentIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(TournamentIden::Id).eq(tournament_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Tournament::from))
    }

    async fn delete(&self, tournament_id: Uuid) -> Result<Option<Tournament>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TournamentIden::Table)
            .and_where(Expr::col(TournamentIden::Id).eq(tournament_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(Tournament::from))
    }
}
