CREATE TABLE twitter_credentials (
    name TEXT PRIMARY KEY,
    consumer_key TEXT UNIQUE NOT NULL,
    consumer_secret TEXT NOT NULL
)