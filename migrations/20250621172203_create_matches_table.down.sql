-- Drop indexes
DROP INDEX IF EXISTS idx_matches_participant2_player;
DROP INDEX IF EXISTS idx_matches_participant1_player;
DROP INDEX IF EXISTS idx_matches_participant2_team;
DROP INDEX IF EXISTS idx_matches_participant1_team;
DROP INDEX IF EXISTS idx_matches_scheduled_date;
DROP INDEX IF EXISTS idx_matches_status;
DROP INDEX IF EXISTS idx_matches_tournament_category;

-- Drop table
DROP TABLE IF EXISTS matches;

-- Drop enums
DROP TYPE IF EXISTS match_type;
DROP TYPE IF EXISTS match_status;