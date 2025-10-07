-- PostgreSQL Migration - Complete Schema
CREATE TABLE IF NOT EXISTS domains (
                                       domain VARCHAR(255) PRIMARY KEY,
    robots_txt TEXT,
    robots_fetched_at TIMESTAMPTZ,
    crawl_delay INTEGER DEFAULT 1000,
    page_count INTEGER DEFAULT 0,
    avg_quality_score DOUBLE PRECISION,
    last_crawled TIMESTAMPTZ,
    crawl_allowed BOOLEAN DEFAULT TRUE
    );

CREATE TABLE IF NOT EXISTS pages (
                                     id BIGSERIAL PRIMARY KEY,
                                     url TEXT UNIQUE NOT NULL,
                                     url_hash VARCHAR(64) UNIQUE NOT NULL,
    domain TEXT NOT NULL,
    title TEXT,
    description TEXT,
    content TEXT,
    content_hash VARCHAR(64),
    quality_score DOUBLE PRECISION DEFAULT 0.0,
    word_count INTEGER DEFAULT 0,
    language VARCHAR(10) DEFAULT 'en',
    crawl_depth INTEGER DEFAULT 0,
    crawled_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMPTZ,
    status_code INTEGER,
    content_type VARCHAR(100),
    content_length INTEGER
    );

CREATE TABLE IF NOT EXISTS links (
                                     id BIGSERIAL PRIMARY KEY,
                                     source_page_id BIGINT NOT NULL,
                                     target_page_id BIGINT,
                                     target_url TEXT NOT NULL,
                                     anchor_text TEXT,
                                     link_position INTEGER DEFAULT 0,
                                     created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                     FOREIGN KEY (source_page_id) REFERENCES pages(id) ON DELETE CASCADE,
    FOREIGN KEY (target_page_id) REFERENCES pages(id) ON DELETE SET NULL
    );

CREATE TABLE IF NOT EXISTS crawl_sessions (
                                              id SERIAL PRIMARY KEY,
                                              started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
                                              ended_at TIMESTAMPTZ,
                                              pages_crawled INTEGER DEFAULT 0,
                                              pages_failed INTEGER DEFAULT 0,
                                              seed_urls TEXT,
                                              config_snapshot TEXT,
                                              status VARCHAR(20) DEFAULT 'running'
    );

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_pages_url_hash ON pages(url_hash);
CREATE INDEX IF NOT EXISTS idx_pages_domain ON pages(domain);
CREATE INDEX IF NOT EXISTS idx_pages_quality ON pages(quality_score DESC);
CREATE INDEX IF NOT EXISTS idx_pages_crawled_at ON pages(crawled_at DESC);
CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_page_id);
CREATE INDEX IF NOT EXISTS idx_links_target_page ON links(target_page_id);
CREATE INDEX IF NOT EXISTS idx_links_target_url ON links(target_url);
CREATE INDEX IF NOT EXISTS idx_domains_last_crawled ON domains(last_crawled);
