CREATE TABLE twitter_accounts (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    username TEXT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    description TEXT,
    location TEXT,
    pinned_tweet_id BIGINT,
    profile_image_url TEXT,
    protected BOOLEAN NOT NULL,
    public_metrics JSONB,
    url TEXT,
    verified BOOLEAN NOT NULL,
    withheld JSONB
)