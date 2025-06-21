-- Drop indexes
DROP INDEX IF EXISTS idx_tournament_categories_team_composition;
DROP INDEX IF EXISTS idx_tournament_categories_tournament_id;

-- Drop table
DROP TABLE IF EXISTS tournament_categories;