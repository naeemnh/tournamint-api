use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::user::{
    EditableUser, NewUser, NewUserProfile, PublicUserProfile, TokenRepository, UpdateUserProfile,
    User, UserProfile, UserProfileRepository, UserRepository, UserToken,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden Definitions ====================

pub enum UserIden {
    Table,
    GoogleId,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}

impl Iden for UserIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                UserIden::Table => "users",
                UserIden::Id => "id",
                UserIden::GoogleId => "google_id",
                UserIden::Name => "name",
                UserIden::Email => "email",
                UserIden::CreatedAt => "created_at",
                UserIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

pub enum UserProfileIden {
    Table,
    Id,
    UserId,
    Bio,
    AvatarUrl,
    Phone,
    DateOfBirth,
    Timezone,
    Language,
    NotificationPreferences,
    PrivacySettings,
    Location,
    Website,
    SocialLinks,
    Preferences,
    IsPublic,
    CreatedAt,
    UpdatedAt,
}

impl Iden for UserProfileIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                UserProfileIden::Table => "user_profiles",
                UserProfileIden::Id => "id",
                UserProfileIden::UserId => "user_id",
                UserProfileIden::Bio => "bio",
                UserProfileIden::AvatarUrl => "avatar_url",
                UserProfileIden::Phone => "phone",
                UserProfileIden::DateOfBirth => "date_of_birth",
                UserProfileIden::Timezone => "timezone",
                UserProfileIden::Language => "language",
                UserProfileIden::NotificationPreferences => "notification_preferences",
                UserProfileIden::PrivacySettings => "privacy_settings",
                UserProfileIden::Location => "location",
                UserProfileIden::Website => "website",
                UserProfileIden::SocialLinks => "social_links",
                UserProfileIden::Preferences => "preferences",
                UserProfileIden::IsPublic => "is_public",
                UserProfileIden::CreatedAt => "created_at",
                UserProfileIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

pub enum UserTokenIden {
    Table,
    RefreshToken,
    UserId,
    ExpiresAt,
}

impl Iden for UserTokenIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                UserTokenIden::Table => "user_tokens",
                UserTokenIden::RefreshToken => "refresh_token",
                UserTokenIden::UserId => "user_id",
                UserTokenIden::ExpiresAt => "expires_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types (with sqlx::FromRow) ====================

#[derive(Debug, FromRow)]
struct UserRow {
    id: Uuid,
    google_id: String,
    email: String,
    name: Option<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            google_id: row.google_id,
            email: row.email,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct UserProfileRow {
    id: Uuid,
    user_id: Uuid,
    bio: Option<String>,
    avatar_url: Option<String>,
    phone: Option<String>,
    date_of_birth: Option<chrono::NaiveDate>,
    timezone: Option<String>,
    language: Option<String>,
    notification_preferences: Option<Value>,
    privacy_settings: Option<Value>,
    location: Option<String>,
    website: Option<String>,
    social_links: Option<Value>,
    preferences: Option<Value>,
    is_public: Option<bool>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<UserProfileRow> for UserProfile {
    fn from(row: UserProfileRow) -> Self {
        UserProfile {
            id: row.id,
            user_id: row.user_id,
            bio: row.bio,
            avatar_url: row.avatar_url,
            phone: row.phone,
            date_of_birth: row.date_of_birth,
            timezone: row.timezone,
            language: row.language,
            notification_preferences: row.notification_preferences,
            privacy_settings: row.privacy_settings,
            location: row.location,
            website: row.website,
            social_links: row.social_links,
            preferences: row.preferences,
            is_public: row.is_public,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct PublicUserProfileRow {
    id: Uuid,
    user_id: Uuid,
    bio: Option<String>,
    avatar_url: Option<String>,
    location: Option<String>,
    website: Option<String>,
    social_links: Option<Value>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<PublicUserProfileRow> for PublicUserProfile {
    fn from(row: PublicUserProfileRow) -> Self {
        PublicUserProfile {
            id: row.id,
            user_id: row.user_id,
            bio: row.bio,
            avatar_url: row.avatar_url,
            location: row.location,
            website: row.website,
            social_links: row.social_links,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ==================== Repository Implementations ====================

pub struct PgUserRepository {
    pool: DbPool,
}

impl PgUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let (sql, _) = Query::select()
            .columns([
                UserIden::Id,
                UserIden::GoogleId,
                UserIden::Name,
                UserIden::Email,
                UserIden::CreatedAt,
                UserIden::UpdatedAt,
            ])
            .from(UserIden::Table)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<UserRow> = sqlx::query_as(&sql)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(User::from).collect())
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                UserIden::Id,
                UserIden::GoogleId,
                UserIden::Name,
                UserIden::Email,
                UserIden::CreatedAt,
                UserIden::UpdatedAt,
            ])
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::Id).eq(user_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(User::from))
    }

    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                UserIden::Id,
                UserIden::GoogleId,
                UserIden::Name,
                UserIden::Email,
                UserIden::CreatedAt,
                UserIden::UpdatedAt,
            ])
            .from(UserIden::Table)
            .and_where(Expr::col(UserIden::GoogleId).eq(google_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(User::from))
    }

    async fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        let (sql, values) = Query::insert()
            .into_table(UserIden::Table)
            .columns([UserIden::GoogleId, UserIden::Name, UserIden::Email])
            .values([
                new_user.google_id.into(),
                new_user.name.into(),
                new_user.email.into(),
            ])
            .unwrap()
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: UserRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(User::from(row))
    }

    async fn update(&self, user_id: Uuid, user_data: EditableUser) -> Result<Option<User>, AppError> {
        let (sql, values) = Query::update()
            .table(UserIden::Table)
            .values([
                (UserIden::Name, user_data.name.into()),
                (UserIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserIden::Id).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(User::from))
    }

    async fn delete(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(UserIden::Table)
            .and_where(Expr::col(UserIden::Id).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(User::from))
    }
}

// ==================== User Profile Repository ====================

pub struct PgUserProfileRepository {
    pool: DbPool,
}

impl PgUserProfileRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserProfileRepository for PgUserProfileRepository {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                UserProfileIden::Id,
                UserProfileIden::UserId,
                UserProfileIden::Bio,
                UserProfileIden::AvatarUrl,
                UserProfileIden::Phone,
                UserProfileIden::DateOfBirth,
                UserProfileIden::Timezone,
                UserProfileIden::Language,
                UserProfileIden::NotificationPreferences,
                UserProfileIden::PrivacySettings,
                UserProfileIden::Location,
                UserProfileIden::Website,
                UserProfileIden::SocialLinks,
                UserProfileIden::Preferences,
                UserProfileIden::IsPublic,
                UserProfileIden::CreatedAt,
                UserProfileIden::UpdatedAt,
            ])
            .from(UserProfileIden::Table)
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn find_public_profile_by_user_id(&self, user_id: Uuid) -> Result<Option<PublicUserProfile>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                UserProfileIden::Id,
                UserProfileIden::UserId,
                UserProfileIden::Bio,
                UserProfileIden::AvatarUrl,
                UserProfileIden::Location,
                UserProfileIden::Website,
                UserProfileIden::SocialLinks,
                UserProfileIden::CreatedAt,
                UserProfileIden::UpdatedAt,
            ])
            .from(UserProfileIden::Table)
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .and_where(Expr::col(UserProfileIden::IsPublic).eq(true))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<PublicUserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(PublicUserProfile::from))
    }

    async fn create(&self, new_profile: NewUserProfile) -> Result<UserProfile, AppError> {
        let (sql, values) = Query::insert()
            .into_table(UserProfileIden::Table)
            .columns([
                UserProfileIden::UserId,
                UserProfileIden::Bio,
                UserProfileIden::AvatarUrl,
                UserProfileIden::Phone,
                UserProfileIden::DateOfBirth,
                UserProfileIden::Timezone,
                UserProfileIden::Language,
                UserProfileIden::NotificationPreferences,
                UserProfileIden::PrivacySettings,
                UserProfileIden::Location,
                UserProfileIden::Website,
                UserProfileIden::SocialLinks,
                UserProfileIden::Preferences,
                UserProfileIden::IsPublic,
            ])
            .values([
                new_profile.user_id.into(),
                new_profile.bio.into(),
                new_profile.avatar_url.into(),
                new_profile.phone.into(),
                new_profile.date_of_birth.into(),
                new_profile.timezone.into(),
                new_profile.language.into(),
                new_profile.notification_preferences.into(),
                new_profile.privacy_settings.into(),
                new_profile.location.into(),
                new_profile.website.into(),
                new_profile.social_links.into(),
                new_profile.preferences.into(),
                new_profile.is_public.into(),
            ])
            .unwrap()
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: UserProfileRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(UserProfile::from(row))
    }

    async fn update(&self, user_id: Uuid, profile_data: UpdateUserProfile) -> Result<Option<UserProfile>, AppError> {
        let mut query = Query::update();
        query.table(UserProfileIden::Table);

        if let Some(bio) = profile_data.bio {
            query.value(UserProfileIden::Bio, bio);
        }
        if let Some(phone) = profile_data.phone {
            query.value(UserProfileIden::Phone, phone);
        }
        if let Some(date_of_birth) = profile_data.date_of_birth {
            query.value(UserProfileIden::DateOfBirth, date_of_birth);
        }
        if let Some(timezone) = profile_data.timezone {
            query.value(UserProfileIden::Timezone, timezone);
        }
        if let Some(language) = profile_data.language {
            query.value(UserProfileIden::Language, language);
        }
        if let Some(location) = profile_data.location {
            query.value(UserProfileIden::Location, location);
        }
        if let Some(website) = profile_data.website {
            query.value(UserProfileIden::Website, website);
        }
        if let Some(social_links) = profile_data.social_links {
            query.value(UserProfileIden::SocialLinks, social_links);
        }
        if let Some(is_public) = profile_data.is_public {
            query.value(UserProfileIden::IsPublic, is_public);
        }

        query.value(UserProfileIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(UserProfileIden::UserId).eq(user_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn update_preferences(&self, user_id: Uuid, preferences: Value) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::Preferences, preferences.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn update_notification_preferences(&self, user_id: Uuid, notification_preferences: Value) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::NotificationPreferences, notification_preferences.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn update_privacy_settings(&self, user_id: Uuid, privacy_settings: Value) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::PrivacySettings, privacy_settings.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn update_avatar(&self, user_id: Uuid, avatar_url: String) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::AvatarUrl, avatar_url.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn remove_avatar(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::AvatarUrl, Option::<String>::None.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(UserProfileIden::Table)
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<UserProfileRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(UserProfile::from))
    }
}

// ==================== Token Repository ====================

pub struct PgTokenRepository {
    pool: DbPool,
}

impl PgTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenRepository for PgTokenRepository {
    async fn upsert_refresh_token(&self, token_data: UserToken) -> Result<(), AppError> {
        let (sql, values) = Query::insert()
            .into_table(UserTokenIden::Table)
            .columns([
                UserTokenIden::UserId,
                UserTokenIden::RefreshToken,
                UserTokenIden::ExpiresAt,
            ])
            .values([
                token_data.user_id.into(),
                token_data.refresh_token.into(),
                token_data.expires_at.into(),
            ])
            .map_err(|e| AppError::InternalError(e.to_string()))?
            .on_conflict(
                sea_query::OnConflict::column(UserTokenIden::UserId)
                    .update_columns([UserTokenIden::RefreshToken, UserTokenIden::ExpiresAt])
                    .to_owned(),
            )
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
