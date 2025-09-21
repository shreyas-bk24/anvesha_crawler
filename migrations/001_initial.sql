-- Initial migration placeholder
-- This will be expanded later when you add database functionality

CREATE TABLE IF NOT EXISTS placeholder (
                                           id SERIAL PRIMARY KEY,
                                           created_at TIMESTAMPTZ DEFAULT NOW()
    );
