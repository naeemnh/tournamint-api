-- Drop view
DROP VIEW IF EXISTS match_scores_summary;

-- Drop indexes
DROP INDEX IF EXISTS idx_match_results_period;
DROP INDEX IF EXISTS idx_match_results_set;
DROP INDEX IF EXISTS idx_match_results_match;

-- Drop table
DROP TABLE IF EXISTS match_results;