-- Add up migration script here
CREATE TABLE IF NOT EXISTS team_members (
    team_id UUID REFERENCES teams (id) ON DELETE CASCADE,
    player_id UUID REFERENCES players (id) ON DELETE CASCADE,
    is_captain BOOLEAN DEFAULT FALSE,
    jersey_number INTEGER,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)