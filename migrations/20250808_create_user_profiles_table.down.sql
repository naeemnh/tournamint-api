-- Add down migration script here
DROP INDEX IF EXISTS idx_user_profiles_is_public;
DROP INDEX IF EXISTS idx_user_profiles_user_id;
DROP TABLE IF EXISTS user_profiles;