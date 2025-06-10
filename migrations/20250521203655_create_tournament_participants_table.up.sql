-- Add up migration script here
CREATE TABLE IF NOT EXISTS tournament_participants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    tournament_id UUID REFERENCES tournaments (id),
    player_id UUID REFERENCES players (id),
    team_id UUID REFERENCES teams (id),
    seed INTEGER,
    status VARCHAR(20) DEFAULT "registered", -- registered, withdrawn, disqualified
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW() ON UPDATE NOW(),
    CHECK (
        (
            player_id IS NOT NULL
            AND team_id IS NULL
        )
        OR (
            player_id IS NULL
            AND team_id IS NOT NULL
        )
    )
);