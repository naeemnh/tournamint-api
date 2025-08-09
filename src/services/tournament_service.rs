use actix_web::{HttpRequest, HttpResponse};
use chrono::Utc;
use oauth2::http::StatusCode;
use serde::Serialize;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::{
    config::DbPool,
    controllers::tournament_controller::TournamentSearchQuery,
    formatters,
    middlewares::auth::get_user_from_token,
    models::tournament::{EditableTournament, NewTournament, Tournament, TournamentFormat, TournamentStatus, SportType},
    models::tournament_category::TournamentCategory,
    models::tournament_registration::RegistrationWithDetails,
    repositories::{TournamentRepository, TournamentCategoryRepository, TournamentRegistrationRepository},
    utils::db::with_transaction,
};

#[derive(Debug, Serialize)]
pub struct TournamentStats {
    pub participants_count: i64,
    pub registrations_count: i64,
    pub categories_count: i64,
    pub matches_played: i64,
    pub prize_pool_total: String,
    pub status: TournamentStatus,
}

#[derive(Debug, Serialize)]
pub struct TournamentDashboard {
    pub tournament: Tournament,
    pub stats: TournamentStats,
    pub recent_registrations: Vec<RegistrationWithDetails>,
    pub categories: Vec<TournamentCategory>,
}

#[derive(Debug, Serialize)]
pub struct ExportData {
    pub format: String,
    pub data: JsonValue,
    pub filename: String,
    pub content_type: String,
}

#[derive(Debug, Serialize)]
pub struct TournamentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub sport_type: SportType,
    pub format: TournamentFormat,
    pub default_settings: JsonValue,
}

impl Default for EditableTournament {
    fn default() -> Self {
        Self {
            name: None,
            description: None,
            sport_type: None,
            format: None,
            status: None,
            start_date: None,
            end_date: None,
            registration_start_date: None,
            registration_end_date: None,
            venue: None,
            max_participants: None,
            entry_fee: None,
            prize_pool: None,
            rules: None,
        }
    }
}

pub struct TournamentService;

impl TournamentService {
    pub async fn create_tournament(pool: &DbPool, data: NewTournament) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::create(tx, data).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::CREATED, tournament, "TOURNAMENT_CREATED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    &error_message,
                    "TOURNAMENT_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_all_tournaments(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::get_all(tx).await })
        })
        .await
        {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_tournament_by_id(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::get_by_id(tx, id).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_FETCHED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_tournaments_by_status(pool: &DbPool, status: TournamentStatus) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::get_by_status(tx, status).await })
        })
        .await
        {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn update_tournament(
        pool: &DbPool,
        id: Uuid,
        data: EditableTournament,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::update(tx, id, data).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_UPDATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_UPDATE_ERROR",
                )
            }
        }
    }

    pub async fn delete_tournament(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_DELETED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_DELETE_ERROR",
                )
            }
        }
    }

    // 1. Search tournaments with filters
    pub async fn search_tournaments(pool: &DbPool, query: TournamentSearchQuery) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::search_tournaments_impl(tx, query).await
            })
        }).await {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "TOURNAMENTS_SEARCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_SEARCH_ERROR",
                )
            }
        }
    }

    // 2. Get tournaments for current user
    pub async fn get_my_tournaments(pool: &DbPool, req: HttpRequest) -> HttpResponse {
        let user = match get_user_from_token(&req).await {
            Ok(user) => user,
            Err(response) => return response,
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRepository::get_by_organizer(tx, user.id).await
            })
        }).await {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "MY_TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "MY_TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    // 3. Get featured tournaments
    pub async fn get_featured_tournaments(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::get_featured_tournaments_impl(tx).await
            })
        }).await {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "FEATURED_TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "FEATURED_TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    // 4. Get upcoming tournaments
    pub async fn get_upcoming_tournaments(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::get_upcoming_tournaments_impl(tx).await
            })
        }).await {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "UPCOMING_TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "UPCOMING_TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    // 5. Publish tournament
    pub async fn publish_tournament(pool: &DbPool, id: Uuid) -> HttpResponse {
        let update_data = EditableTournament {
            status: Some(TournamentStatus::RegistrationOpen),
            ..Default::default()
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRepository::update(tx, id, update_data).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_PUBLISHED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_PUBLISH_ERROR",
                )
            }
        }
    }

    // 6. Start tournament
    pub async fn start_tournament(pool: &DbPool, id: Uuid) -> HttpResponse {
        let update_data = EditableTournament {
            status: Some(TournamentStatus::InProgress),
            ..Default::default()
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRepository::update(tx, id, update_data).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_STARTED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_START_ERROR",
                )
            }
        }
    }

    // 7. Complete tournament
    pub async fn complete_tournament(pool: &DbPool, id: Uuid) -> HttpResponse {
        let update_data = EditableTournament {
            status: Some(TournamentStatus::Completed),
            ..Default::default()
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRepository::update(tx, id, update_data).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_COMPLETED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_COMPLETE_ERROR",
                )
            }
        }
    }

    // 8. Cancel tournament
    pub async fn cancel_tournament(pool: &DbPool, id: Uuid, reason: Option<String>) -> HttpResponse {
        let mut rules = serde_json::Map::new();
        if let Some(cancel_reason) = reason {
            rules.insert("cancellation_reason".to_string(), JsonValue::String(cancel_reason));
        }

        let update_data = EditableTournament {
            status: Some(TournamentStatus::Cancelled),
            rules: Some(JsonValue::Object(rules)),
            ..Default::default()
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRepository::update(tx, id, update_data).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_CANCELLED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_CANCEL_ERROR",
                )
            }
        }
    }

    // 9. Get tournament statistics
    pub async fn get_tournament_stats(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::get_tournament_stats_impl(tx, id).await
            })
        }).await {
            Ok(stats) => {
                formatters::success_response(StatusCode::OK, stats, "TOURNAMENT_STATS_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_STATS_ERROR",
                )
            }
        }
    }

    // 10. Get tournament participants
    pub async fn get_tournament_participants(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRegistrationRepository::get_by_tournament(tx, id).await
            })
        }).await {
            Ok(participants) => {
                formatters::success_response(StatusCode::OK, participants, "TOURNAMENT_PARTICIPANTS_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_PARTICIPANTS_ERROR",
                )
            }
        }
    }

    // 11. Export tournament
    pub async fn export_tournament(pool: &DbPool, id: Uuid, format: String) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::export_tournament_impl(tx, id, format).await
            })
        }).await {
            Ok(export_data) => {
                formatters::success_response(StatusCode::OK, export_data, "TOURNAMENT_EXPORTED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_EXPORT_ERROR",
                )
            }
        }
    }

    // 12. Duplicate tournament
    pub async fn duplicate_tournament(pool: &DbPool, id: Uuid, new_name: String) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::duplicate_tournament_impl(tx, id, new_name).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::CREATED, tournament, "TOURNAMENT_DUPLICATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_DUPLICATE_ERROR",
                )
            }
        }
    }

    // 13. Get tournament templates
    pub async fn get_tournament_templates(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::get_tournament_templates_impl(tx).await
            })
        }).await {
            Ok(templates) => {
                formatters::success_response(StatusCode::OK, templates, "TOURNAMENT_TEMPLATES_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_TEMPLATES_ERROR",
                )
            }
        }
    }

    // 14. Create from template
    pub async fn create_from_template(pool: &DbPool, template_id: Uuid, data: NewTournament) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::create_from_template_impl(tx, template_id, data).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::CREATED, tournament, "TOURNAMENT_CREATED_FROM_TEMPLATE")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    &e.to_string(),
                    "TOURNAMENT_TEMPLATE_ERROR",
                )
            }
        }
    }

    // 15. Get tournament dashboard
    pub async fn get_tournament_dashboard(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::get_tournament_dashboard_impl(tx, id).await
            })
        }).await {
            Ok(dashboard) => {
                formatters::success_response(StatusCode::OK, dashboard, "TOURNAMENT_DASHBOARD_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_DASHBOARD_ERROR",
                )
            }
        }
    }

    // 16. Update tournament settings
    pub async fn update_tournament_settings(pool: &DbPool, id: Uuid, settings: JsonValue) -> HttpResponse {
        let update_data = EditableTournament {
            rules: Some(settings),
            ..Default::default()
        };

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRepository::update(tx, id, update_data).await
            })
        }).await {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_SETTINGS_UPDATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TOURNAMENT_SETTINGS_ERROR",
                )
            }
        }
    }

    // 17. Get tournament categories
    pub async fn get_tournament_categories(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentCategoryRepository::get_by_tournament(tx, id).await
            })
        }).await {
            Ok(categories) => {
                formatters::success_response(StatusCode::OK, categories, "TOURNAMENT_CATEGORIES_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_CATEGORIES_ERROR",
                )
            }
        }
    }

    // 18. Get tournament registrations
    pub async fn get_tournament_registrations(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRegistrationRepository::get_by_tournament(tx, id).await
            })
        }).await {
            Ok(registrations) => {
                formatters::success_response(StatusCode::OK, registrations, "TOURNAMENT_REGISTRATIONS_FETCHED")
            }
            Err(e) => {
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "TOURNAMENT_REGISTRATIONS_ERROR",
                )
            }
        }
    }

    // Helper implementation methods
    async fn search_tournaments_impl(
        tx: &mut sqlx::PgConnection,
        query: TournamentSearchQuery,
    ) -> Result<Vec<Tournament>, sqlx::Error> {
        use sea_query::{Expr, PostgresQueryBuilder, Query};
        use sea_query::extension::postgres::PgExpr;
        use sea_query_binder::SqlxBinder;
        use crate::models::tournament::TournamentIden;

        let mut sql_query = Query::select();
        sql_query
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
            .from(TournamentIden::Table);

        // Add filters
        if let Some(name) = query.name {
            sql_query.and_where(Expr::col(TournamentIden::Name).ilike(format!("%{}%", name)));
        }
        if let Some(sport_type) = query.sport_type {
            sql_query.and_where(Expr::col(TournamentIden::SportType).eq(format!("\"{}\"", sport_type)));
        }
        if let Some(status) = query.status {
            sql_query.and_where(Expr::col(TournamentIden::Status).eq(format!("\"{}\"", status)));
        }
        if let Some(venue) = query.location {
            sql_query.and_where(Expr::col(TournamentIden::Venue).ilike(format!("%{}%", venue)));
        }
        if let Some(date_from) = query.date_from {
            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&date_from) {
                sql_query.and_where(Expr::col(TournamentIden::StartDate).gte(date.with_timezone(&Utc)));
            }
        }
        if let Some(date_to) = query.date_to {
            if let Ok(date) = chrono::DateTime::parse_from_rfc3339(&date_to) {
                sql_query.and_where(Expr::col(TournamentIden::EndDate).lte(date.with_timezone(&Utc)));
            }
        }

        sql_query.order_by(TournamentIden::StartDate, sea_query::Order::Desc);

        if let Some(limit) = query.limit {
            sql_query.limit(limit as u64);
        }
        if let Some(offset) = query.offset {
            sql_query.offset(offset as u64);
        }

        let (sql, values) = sql_query.build_sqlx(PostgresQueryBuilder);
        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    async fn get_featured_tournaments_impl(
        tx: &mut sqlx::PgConnection,
    ) -> Result<Vec<Tournament>, sqlx::Error> {
        use sea_query::{Expr, PostgresQueryBuilder, Query};
        use sea_query_binder::SqlxBinder;
        use crate::models::tournament::TournamentIden;

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
            .and_where(Expr::col(TournamentIden::Status).ne(serde_json::to_string(&TournamentStatus::Draft).unwrap()))
            .and_where(Expr::col(TournamentIden::Status).ne(serde_json::to_string(&TournamentStatus::Cancelled).unwrap()))
            .order_by(TournamentIden::PrizePool, sea_query::Order::Desc)
            .order_by(TournamentIden::StartDate, sea_query::Order::Asc)
            .limit(10)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    async fn get_upcoming_tournaments_impl(
        tx: &mut sqlx::PgConnection,
    ) -> Result<Vec<Tournament>, sqlx::Error> {
        use sea_query::{Expr, PostgresQueryBuilder, Query};
        use sea_query_binder::SqlxBinder;
        use crate::models::tournament::TournamentIden;

        let now = Utc::now();
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
            .and_where(Expr::col(TournamentIden::StartDate).gt(now))
            .and_where(Expr::col(TournamentIden::Status).ne(serde_json::to_string(&TournamentStatus::Cancelled).unwrap()))
            .order_by(TournamentIden::StartDate, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    async fn get_tournament_stats_impl(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
    ) -> Result<TournamentStats, sqlx::Error> {
        // Get tournament first to get status
        let tournament = TournamentRepository::get_by_id(tx, tournament_id).await?;

        // Get participants count
        let participants_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT CASE WHEN team_id IS NOT NULL THEN team_id ELSE player_id END) FROM tournament_registrations tr INNER JOIN tournament_categories tc ON tc.id = tr.tournament_category_id WHERE tc.tournament_id = $1"
        )
        .bind(tournament_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or(0);

        // Get registrations count
        let registrations_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tournament_registrations tr INNER JOIN tournament_categories tc ON tc.id = tr.tournament_category_id WHERE tc.tournament_id = $1"
        )
        .bind(tournament_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or(0);

        // Get categories count
        let categories_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM tournament_categories WHERE tournament_id = $1"
        )
        .bind(tournament_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or(0);

        // Get matches played (placeholder - assuming you have a matches table)
        let matches_played: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM matches m INNER JOIN tournament_categories tc ON tc.id = m.tournament_category_id WHERE tc.tournament_id = $1 AND m.status = 'completed'"
        )
        .bind(tournament_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or(0);

        let prize_pool_total = tournament.prize_pool
            .map(|p| p.to_string())
            .unwrap_or_else(|| "0.00".to_string());

        Ok(TournamentStats {
            participants_count,
            registrations_count,
            categories_count,
            matches_played,
            prize_pool_total,
            status: tournament.status,
        })
    }

    async fn export_tournament_impl(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        format: String,
    ) -> Result<ExportData, sqlx::Error> {
        let tournament = TournamentRepository::get_by_id(tx, tournament_id).await?;
        let categories = TournamentCategoryRepository::get_by_tournament(tx, tournament_id).await?;
        let registrations = TournamentRegistrationRepository::get_by_tournament(tx, tournament_id).await?;

        let mut export_data = serde_json::Map::new();
        export_data.insert("tournament".to_string(), serde_json::to_value(&tournament).unwrap());
        export_data.insert("categories".to_string(), serde_json::to_value(categories).unwrap());
        export_data.insert("registrations".to_string(), serde_json::to_value(registrations).unwrap());
        export_data.insert("exported_at".to_string(), serde_json::to_value(Utc::now()).unwrap());

        let (content_type, filename) = match format.as_str() {
            "csv" => ("text/csv".to_string(), format!("{}_export.csv", tournament.name)),
            "pdf" => ("application/pdf".to_string(), format!("{}_export.pdf", tournament.name)),
            _ => ("application/json".to_string(), format!("{}_export.json", tournament.name)),
        };

        Ok(ExportData {
            format: format.clone(),
            data: JsonValue::Object(export_data),
            filename,
            content_type,
        })
    }

    async fn duplicate_tournament_impl(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        new_name: String,
    ) -> Result<Tournament, sqlx::Error> {
        let original = TournamentRepository::get_by_id(tx, tournament_id).await?;
        
        let new_tournament = NewTournament {
            name: new_name,
            description: original.description.clone(),
            sport_type: original.sport_type,
            format: original.format,
            start_date: original.start_date + chrono::Duration::weeks(1), // Default to next week
            end_date: original.end_date + chrono::Duration::weeks(1),
            registration_start_date: original.registration_start_date.map(|d| d + chrono::Duration::days(7)),
            registration_end_date: original.registration_end_date.map(|d| d + chrono::Duration::days(7)),
            venue: original.venue.clone(),
            max_participants: original.max_participants,
            entry_fee: original.entry_fee,
            prize_pool: original.prize_pool,
            rules: original.rules.as_ref().map(|r| r.0.clone()),
            organizer_id: original.organizer_id,
        };

        TournamentRepository::create(tx, new_tournament).await
    }

    async fn get_tournament_templates_impl(
        _tx: &mut sqlx::PgConnection,
    ) -> Result<Vec<TournamentTemplate>, sqlx::Error> {
        // For now, return predefined templates. In a real app, you'd store these in DB
        let templates = vec![
            TournamentTemplate {
                id: "single_elimination".to_string(),
                name: "Single Elimination".to_string(),
                description: "Standard single elimination tournament".to_string(),
                sport_type: SportType::Basketball,
                format: TournamentFormat::Elimination,
                default_settings: serde_json::json!({
                    "bracket_type": "single",
                    "seeding": "random",
                    "third_place": false
                }),
            },
            TournamentTemplate {
                id: "round_robin".to_string(),
                name: "Round Robin".to_string(),
                description: "Everyone plays everyone tournament format".to_string(),
                sport_type: SportType::TableTennis,
                format: TournamentFormat::RoundRobin,
                default_settings: serde_json::json!({
                    "points_win": 3,
                    "points_draw": 1,
                    "points_loss": 0
                }),
            },
        ];
        
        Ok(templates)
    }

    async fn create_from_template_impl(
        tx: &mut sqlx::PgConnection,
        _template_id: Uuid,
        mut tournament_data: NewTournament,
    ) -> Result<Tournament, sqlx::Error> {
        // For now, just create the tournament as provided
        // In a real implementation, you'd apply template settings
        tournament_data.format = TournamentFormat::Elimination;
        
        TournamentRepository::create(tx, tournament_data).await
    }

    async fn get_tournament_dashboard_impl(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
    ) -> Result<TournamentDashboard, sqlx::Error> {
        let tournament = TournamentRepository::get_by_id(tx, tournament_id).await?;
        let stats = Self::get_tournament_stats_impl(tx, tournament_id).await?;
        let categories = TournamentCategoryRepository::get_by_tournament(tx, tournament_id).await?;
        let recent_registrations = TournamentRegistrationRepository::get_by_tournament(tx, tournament_id).await?
            .into_iter()
            .take(10)
            .collect();

        Ok(TournamentDashboard {
            tournament,
            stats,
            recent_registrations,
            categories,
        })
    }
}