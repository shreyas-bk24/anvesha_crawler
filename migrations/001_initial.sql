-- migrations/001_initial.sql
-- Core page storage
CREATE TABLE IF NOT EXISTS pages (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     url TEXT UNIQUE NOT NULL,
                                     url_hash TEXT NOT NULL,
                                     domain TEXT NOT NULL,
                                     title TEXT,
                                     description TEXT,
                                     content TEXT NOT NULL,
                                     content_hash TEXT NOT NULL,
                                     quality_score REAL NOT NULL DEFAULT 0.0,
                                     word_count INTEGER NOT NULL DEFAULT 0,
                                     language TEXT DEFAULT 'en',
                                     crawl_depth INTEGER NOT NULL DEFAULT 0,
                                     crawled_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                     last_modified TIMESTAMP,
                                     status_code INTEGER NOT NULL DEFAULT 200,
                                     content_type TEXT DEFAULT 'text/html',
                                     content_length INTEGER NOT NULL DEFAULT 0
);

-- Link graph for PageRank
CREATE TABLE IF NOT EXISTS links (
                                     id INTEGER PRIMARY KEY AUTOINCREMENT,
                                     source_page_id INTEGER NOT NULL,
                                     target_url TEXT NOT NULL,
                                     target_page_id INTEGER, -- NULL if target not crawled yet
                                     anchor_text TEXT,
                                     link_position INTEGER DEFAULT 0,
                                     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                     FOREIGN KEY (source_page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (target_page_id) REFERENCES pages(id) ON DELETE SET NULL
    );

-- Crawl sessions for tracking
CREATE TABLE IF NOT EXISTS crawl_sessions (
                                              id INTEGER PRIMARY KEY AUTOINCREMENT,
                                              started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                              ended_at TIMESTAMP,
                                              pages_crawled INTEGER DEFAULT 0,
                                              pages_failed INTEGER DEFAULT 0,
                                              seed_urls TEXT, -- JSON array
                                              config_snapshot TEXT, -- JSON config
                                              status TEXT DEFAULT 'running' -- 'running', 'completed', 'failed'
);

-- Domain metadata
CREATE TABLE IF NOT EXISTS domains (
                                       domain TEXT PRIMARY KEY,
                                       robots_txt TEXT,
                                       robots_fetched_at TIMESTAMP,
                                       crawl_delay INTEGER DEFAULT 1000, -- milliseconds
                                       page_count INTEGER DEFAULT 0,
                                       avg_quality_score REAL,
                                       last_crawled TIMESTAMP,
                                       crawl_allowed BOOLEAN DEFAULT TRUE
);
