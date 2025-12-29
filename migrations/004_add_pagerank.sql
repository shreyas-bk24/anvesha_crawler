ALTER TABLE pages ADD COLUMN IF NOT EXISTS pagerank REAL DEFAULT 0.0;
CREATE INDEX IF NOT EXISTS idx_pagerank ON pages(pagerank DESC);

-- Verify column exists
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'pages' AND column_name = 'pagerank'
    ) THEN
        RAISE EXCEPTION 'PageRank column was not created successfully';
END IF;
END $$;