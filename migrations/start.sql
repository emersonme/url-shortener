CREATE TABLE IF NOT EXISTS urls (
    id SERIAL PRIMARY KEY,
    short_code VARCHAR(10) UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    clicks BIGINT DEFAULT 0
);
 
CREATE INDEX IF NOT EXISTS idx_short_code ON urls(short_code);