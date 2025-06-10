-- Add up migration script here
CREATE TABLE IF NOT EXISTS matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    tournament_id UUID NOT NULL REFERENCES tournaments (id) ON DELETE CASCADE,
    round_id UUID REFERENCES tournament_rounds (id) ON DELETE SET NULL,
    participant1_id UUID REFERENCES tournament_participants (id) ON DELETE SET NULL,
    participant_1_temp_name VARCHAR(255) DEFAULT "Team 1",
    participant2_id UUID REFERENCES tournament_participants (id) ON DELETE SET NULL,
    participant_2_temp_name VARCHAR(255) DEFAULT "Team 2",
    scheduled_time TIMESTAMPTZ,
    venue_id INTEGER REFERENCES venues (id) ON DELETE SET NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'scheduled', -- scheduled, in_progress, completed, postponed, cancelled
    sets INTEGER,
    winner_id UUID REFERENCES tournament_participants (id) ON DELETE SET NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW() ON UPDATE NOW(),
    CHECK (
        participant1_id != participant2_id
        OR participant1_id IS NULL
        OR participant2_id IS NULL
    )
);