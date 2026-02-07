-- Create tournament categories table
CREATE TABLE tournament_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    tournament_id UUID NOT NULL REFERENCES tournaments (id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    team_composition team_composition NOT NULL,
    min_participants INT NOT NULL DEFAULT 2,
    max_participants INT,
    entry_fee DECIMAL(10, 2),
    prize_distribution JSONB,
    rules JSONB,
    constraints JSONB, -- For time constraints, point systems, etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_participant_limits CHECK (
        max_participants IS NULL
        OR max_participants >= min_participants
    ),
    UNIQUE (tournament_id, name)
);

-- Create indexes
CREATE INDEX idx_tournament_categories_tournament_id ON tournament_categories (tournament_id);

CREATE INDEX idx_tournament_categories_team_composition ON tournament_categories (team_composition);