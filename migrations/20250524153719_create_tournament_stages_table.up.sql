-- Add up migration script here
CREATE TABLE tournament_stages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(255) NOT NULL,
    tournament_id UUID NOT NULL REFERENCES tournaments (id) ON DELETE CASCADE,
    is_knockout BOOLEAN NOT NULL DEFAULT FALSE,
    stage_order INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() ON UPDATE NOW(),
    UNIQUE (tournament_id, stage_order)
)