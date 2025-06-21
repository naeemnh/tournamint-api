-- Drop indexes
DROP INDEX IF EXISTS idx_registrations_payment_status;
DROP INDEX IF EXISTS idx_registrations_status;
DROP INDEX IF EXISTS idx_registrations_player;
DROP INDEX IF EXISTS idx_registrations_team;
DROP INDEX IF EXISTS idx_registrations_tournament_category;

-- Drop table
DROP TABLE IF EXISTS tournament_registrations;

-- Drop enums
DROP TYPE IF EXISTS payment_status;
DROP TYPE IF EXISTS registration_status;