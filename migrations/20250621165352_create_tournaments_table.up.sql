-- Create tournaments table
CREATE TABLE tournaments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    sport_type sport_type NOT NULL,
    format tournament_format NOT NULL,
    status tournament_status NOT NULL DEFAULT 'draft',
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    registration_start_date TIMESTAMPTZ,
    registration_end_date TIMESTAMPTZ,
    venue VARCHAR(500),
    max_participants INT,
    entry_fee DECIMAL(10, 2),
    prize_pool DECIMAL(10, 2),
    rules JSONB,
    organizer_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_dates CHECK (end_date > start_date),
    CONSTRAINT valid_registration_dates CHECK (
        registration_end_date IS NULL OR 
        registration_start_date IS NULL OR 
        registration_end_date > registration_start_date
    )
);

-- Create indexes
CREATE INDEX idx_tournaments_sport_type ON tournaments(sport_type);
CREATE INDEX idx_tournaments_status ON tournaments(status);
CREATE INDEX idx_tournaments_start_date ON tournaments(start_date);
CREATE INDEX idx_tournaments_organizer_id ON tournaments(organizer_id);