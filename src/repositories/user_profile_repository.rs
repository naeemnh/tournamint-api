use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::user_profile::{
    NewUserProfile, PublicUserProfile, UpdateUserProfile, UserProfile, UserProfileIden,
};

pub struct UserProfileRepository;

impl UserProfileRepository {
    pub async fn find_by_user_id(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn find_public_profile_by_user_id(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Option<PublicUserProfile>, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn create(
        tx: &mut PgConnection,
        new_profile: NewUserProfile,
    ) -> Result<UserProfile, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn update(
        tx: &mut PgConnection,
        user_id: Uuid,
        profile_data: UpdateUserProfile,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let mut query = Query::update();
        query.table(UserProfileIden::Table);

        // Only update fields that are provided
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

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn update_preferences(
        tx: &mut PgConnection,
        user_id: Uuid,
        preferences: Value,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::Preferences, preferences.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn update_notification_preferences(
        tx: &mut PgConnection,
        user_id: Uuid,
        notification_preferences: Value,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (
                    UserProfileIden::NotificationPreferences,
                    notification_preferences.into(),
                ),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn update_privacy_settings(
        tx: &mut PgConnection,
        user_id: Uuid,
        privacy_settings: Value,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::PrivacySettings, privacy_settings.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn update_avatar(
        tx: &mut PgConnection,
        user_id: Uuid,
        avatar_url: String,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::AvatarUrl, avatar_url.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn remove_avatar(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(UserProfileIden::Table)
            .values([
                (UserProfileIden::AvatarUrl, Option::<String>::None.into()),
                (UserProfileIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn delete_by_user_id(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Option<UserProfile>, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(UserProfileIden::Table)
            .and_where(Expr::col(UserProfileIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }
}