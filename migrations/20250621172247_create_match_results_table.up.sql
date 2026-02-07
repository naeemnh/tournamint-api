-- Create match results table
CREATE TABLE match_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,

-- Scores can be stored in different formats depending on the sport
-- For set-based games (tennis, volleyball)
set_number INT,
participant1_score INT,
participant2_score INT,

-- For time-based games (basketball quarters, football halves)
period_number INT,
period_name VARCHAR(50), -- e.g., "1st Quarter", "2nd Half", "Overtime"

-- For detailed scoring events
scoring_data JSONB, -- Flexible structure for sport-specific data

-- Statistics
participant1_stats JSONB, -- Sport-specific statistics
    participant2_stats JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_match_results_match ON match_results (match_id);

CREATE INDEX idx_match_results_set ON match_results (match_id, set_number)
WHERE
    set_number IS NOT NULL;

CREATE INDEX idx_match_results_period ON match_results (match_id, period_number)
WHERE
    period_number IS NOT NULL;

-- Create a summary view for match scores
CREATE OR REPLACE VIEW match_scores_summary AS
SELECT
    m.id as match_id,
    m.tournament_category_id,
    m.match_status,
    COALESCE(
        SUM(
            CASE
                WHEN mr.participant1_score > mr.participant2_score THEN 1
                ELSE 0
            END
        ),
        0
    ) as participant1_sets_won,
    COALESCE(
        SUM(
            CASE
                WHEN mr.participant2_score > mr.participant1_score THEN 1
                ELSE 0
            END
        ),
        0
    ) as participant2_sets_won,
    COALESCE(SUM(mr.participant1_score), 0) as participant1_total_points,
    COALESCE(SUM(mr.participant2_score), 0) as participant2_total_points
FROM matches m
    LEFT JOIN match_results mr ON m.id = mr.match_id
GROUP BY
    m.id,
    m.tournament_category_id,
    m.match_status;