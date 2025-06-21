-- Drop indexes
DROP INDEX IF EXISTS idx_tournaments_organizer_id;
DROP INDEX IF EXISTS idx_tournaments_start_date;
DROP INDEX IF EXISTS idx_tournaments_status;
DROP INDEX IF EXISTS idx_tournaments_sport_type;

-- Drop table
DROP TABLE IF EXISTS tournaments;