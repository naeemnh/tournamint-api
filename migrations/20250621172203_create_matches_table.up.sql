-- Create match status enum
CREATE TYPE match_status AS ENUM (
    'scheduled',
    'in_progress',
    'completed',
    'cancelled',
    'postponed',
    'forfeited',
    'bye'
);

-- Create match type enum
CREATE TYPE match_type AS ENUM (
    'group_stage',
    'round_of_128',
    'round_of_64',
    'round_of_32',
    'round_of_16',
    'quarter_final',
    'semi_final',
    'third_place',
    'final',
    'qualifying',
    'playoff'
);

-- Create matches table
CREATE TABLE matches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tournament_category_id UUID NOT NULL REFERENCES tournament_categories(id) ON DELETE CASCADE,
    
    -- Participants (could be teams or players depending on the category)
    participant1_team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    participant1_player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    participant1_partner_id UUID REFERENCES players(id) ON DELETE CASCADE, -- For doubles
    
    participant2_team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    participant2_player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    participant2_partner_id UUID REFERENCES players(id) ON DELETE CASCADE, -- For doubles
    
    match_type match_type NOT NULL,
    match_status match_status NOT NULL DEFAULT 'scheduled',
    round_number INT,
    match_number INT, -- Number within the round
    
    scheduled_date TIMESTAMPTZ NOT NULL,
    actual_start_date TIMESTAMPTZ,
    actual_end_date TIMESTAMPTZ,
    
    venue VARCHAR(500),
    court_number VARCHAR(50),
    
    -- Result summary (details in match_results table)
    winner_participant INT, -- 1 or 2, NULL for draws
    is_draw BOOLEAN DEFAULT FALSE,
    
    -- Officials
    referee_name VARCHAR(255),
    umpire_name VARCHAR(255),
    
    notes TEXT,
    metadata JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Ensure valid participants based on tournament category type
    CONSTRAINT valid_participants CHECK (
        -- Team match
        (participant1_team_id IS NOT NULL AND participant2_team_id IS NOT NULL AND 
         participant1_player_id IS NULL AND participant2_player_id IS NULL) OR
        -- Singles match
        (participant1_player_id IS NOT NULL AND participant2_player_id IS NOT NULL AND 
         participant1_team_id IS NULL AND participant2_team_id IS NULL AND
         participant1_partner_id IS NULL AND participant2_partner_id IS NULL) OR
        -- Doubles match
        (participant1_player_id IS NOT NULL AND participant2_player_id IS NOT NULL AND 
         participant1_partner_id IS NOT NULL AND participant2_partner_id IS NOT NULL AND
         participant1_team_id IS NULL AND participant2_team_id IS NULL)
    ),
    CONSTRAINT valid_winner CHECK (winner_participant IN (1, 2) OR winner_participant IS NULL)
);

-- Create indexes
CREATE INDEX idx_matches_tournament_category ON matches(tournament_category_id);
CREATE INDEX idx_matches_status ON matches(match_status);
CREATE INDEX idx_matches_scheduled_date ON matches(scheduled_date);
CREATE INDEX idx_matches_participant1_team ON matches(participant1_team_id) WHERE participant1_team_id IS NOT NULL;
CREATE INDEX idx_matches_participant2_team ON matches(participant2_team_id) WHERE participant2_team_id IS NOT NULL;
CREATE INDEX idx_matches_participant1_player ON matches(participant1_player_id) WHERE participant1_player_id IS NOT NULL;
CREATE INDEX idx_matches_participant2_player ON matches(participant2_player_id) WHERE participant2_player_id IS NOT NULL;