use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::tournament::{
    EditableTournament, NewTournament, Tournament, TournamentIden, TournamentStatus,
};

pub struct TournamentRepository;

impl TournamentRepository {
    pub async fn create(
        tx: &mut PgConnection,
        data: NewTournament,
    ) -> Result<Tournament, sqlx::Error> {
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
                data.name.into(),
                data.description.into(),
                serde_json::to_string(&data.sport_type).unwrap().into(),
                serde_json::to_string(&data.format).unwrap().into(),
                data.start_date.into(),
                data.end_date.into(),
                data.registration_start_date.into(),
                data.registration_end_date.into(),
                data.venue.into(),
                data.max_participants.into(),
                data.entry_fee.into(),
                data.prize_pool.into(),
                data.rules.into(),
                data.organizer_id.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_all(tx: &mut PgConnection) -> Result<Vec<Tournament>, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_by_id(tx: &mut PgConnection, id: Uuid) -> Result<Tournament, sqlx::Error> {
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
            .and_where(Expr::col(TournamentIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_status(
        tx: &mut PgConnection,
        status: TournamentStatus,
    ) -> Result<Vec<Tournament>, sqlx::Error> {
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
            .and_where(
                Expr::col(TournamentIden::Status).eq(serde_json::to_string(&status).unwrap()),
            )
            .order_by(TournamentIden::StartDate, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        data: EditableTournament,
    ) -> Result<Tournament, sqlx::Error> {
        let mut query = Query::update();
        query.table(TournamentIden::Table);

        if let Some(name) = data.name {
            query.value(TournamentIden::Name, name);
        }
        if let Some(description) = data.description {
            query.value(TournamentIden::Description, description);
        }
        if let Some(sport_type) = data.sport_type {
            query.value(
                TournamentIden::SportType,
                serde_json::to_string(&sport_type).unwrap(),
            );
        }
        if let Some(format) = data.format {
            query.value(
                TournamentIden::Format,
                serde_json::to_string(&format).unwrap(),
            );
        }
        if let Some(status) = data.status {
            query.value(
                TournamentIden::Status,
                serde_json::to_string(&status).unwrap(),
            );
        }
        if let Some(start_date) = data.start_date {
            query.value(TournamentIden::StartDate, start_date);
        }
        if let Some(end_date) = data.end_date {
            query.value(TournamentIden::EndDate, end_date);
        }
        if let Some(registration_start_date) = data.registration_start_date {
            query.value(
                TournamentIden::RegistrationStartDate,
                registration_start_date,
            );
        }
        if let Some(registration_end_date) = data.registration_end_date {
            query.value(TournamentIden::RegistrationEndDate, registration_end_date);
        }
        if let Some(venue) = data.venue {
            query.value(TournamentIden::Venue, venue);
        }
        if let Some(max_participants) = data.max_participants {
            query.value(TournamentIden::MaxParticipants, max_participants);
        }
        if let Some(entry_fee) = data.entry_fee {
            query.value(TournamentIden::EntryFee, entry_fee);
        }
        if let Some(prize_pool) = data.prize_pool {
            query.value(TournamentIden::PrizePool, prize_pool);
        }
        if let Some(rules) = data.rules {
            query.value(TournamentIden::Rules, rules);
        }

        query.value(TournamentIden::UpdatedAt, Utc::now());

        let (sql, values) = query
            .and_where(Expr::col(TournamentIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete(tx: &mut PgConnection, id: Uuid) -> Result<Tournament, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(TournamentIden::Table)
            .and_where(Expr::col(TournamentIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_organizer(
        tx: &mut PgConnection,
        organizer_id: Uuid,
    ) -> Result<Vec<Tournament>, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }
}
