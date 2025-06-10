-- Add up migration script here
CREATE TABLE IF NOT EXISTS tournament_rounds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    stage_id UUID NOT NULL REFERENCES tournament_stages (id) ON DELETE CASCADE,
    description TEXT,
    round_order INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() ON UPDATE NOW(),
    UNIQUE (stage_id, round_order)
);