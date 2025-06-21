-- Create registration status enum
CREATE TYPE registration_status AS ENUM (
    'pending',
    'approved',
    'rejected',
    'withdrawn',
    'waitlisted'
);

-- Create payment status enum
CREATE TYPE payment_status AS ENUM (
    'pending',
    'completed',
    'failed',
    'refunded',
    'waived'
);

-- Create tournament registrations table
CREATE TABLE tournament_registrations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tournament_category_id UUID NOT NULL REFERENCES tournament_categories(id) ON DELETE CASCADE,
    -- For team sports
    team_id UUID REFERENCES teams(id) ON DELETE CASCADE,
    -- For individual sports (singles)
    player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    -- For doubles/mixed doubles (store partner)
    partner_player_id UUID REFERENCES players(id) ON DELETE CASCADE,
    
    registration_status registration_status NOT NULL DEFAULT 'pending',
    payment_status payment_status NOT NULL DEFAULT 'pending',
    registration_date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    approval_date TIMESTAMPTZ,
    payment_date TIMESTAMPTZ,
    payment_amount DECIMAL(10, 2),
    payment_reference VARCHAR(255),
    notes TEXT,
    metadata JSONB,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Ensure either team_id or player_id is set (but not both)
    CONSTRAINT valid_participant CHECK (
        (team_id IS NOT NULL AND player_id IS NULL AND partner_player_id IS NULL) OR
        (team_id IS NULL AND player_id IS NOT NULL)
    ),
    -- Ensure partner is only set for non-team registrations
    CONSTRAINT valid_partner CHECK (
        (partner_player_id IS NULL) OR 
        (partner_player_id IS NOT NULL AND team_id IS NULL AND player_id IS NOT NULL)
    ),
    -- Prevent duplicate registrations
    UNIQUE(tournament_category_id, team_id),
    UNIQUE(tournament_category_id, player_id, partner_player_id)
);

-- Create indexes
CREATE INDEX idx_registrations_tournament_category ON tournament_registrations(tournament_category_id);
CREATE INDEX idx_registrations_team ON tournament_registrations(team_id) WHERE team_id IS NOT NULL;
CREATE INDEX idx_registrations_player ON tournament_registrations(player_id) WHERE player_id IS NOT NULL;
CREATE INDEX idx_registrations_status ON tournament_registrations(registration_status);
CREATE INDEX idx_registrations_payment_status ON tournament_registrations(payment_status);