-- Add up migration script here
CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
    name VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID REFERENCES users UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);