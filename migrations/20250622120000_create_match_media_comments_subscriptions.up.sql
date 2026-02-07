-- Match media (videos, photos)
CREATE TABLE match_media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    media_type VARCHAR(50) NOT NULL,
    file_url TEXT NOT NULL,
    thumbnail_url TEXT,
    file_size BIGINT,
    duration INT,
    uploaded_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_match_media_match_id ON match_media(match_id);

-- Match comments
CREATE TABLE match_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    comment TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_match_comments_match_id ON match_comments(match_id);

-- Match subscriptions (users subscribing to match updates)
CREATE TABLE match_subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_preferences JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(match_id, user_id)
);

CREATE INDEX idx_match_subscriptions_match_id ON match_subscriptions(match_id);
CREATE INDEX idx_match_subscriptions_user_id ON match_subscriptions(user_id);
