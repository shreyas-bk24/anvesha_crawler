// Database connection and management

use sqlx::{PgPool, Pool, Postgres, Row};
use std::path::Path;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, warn, error};
use crate::storage::Result;

pub type DatabasePool = Pool<Postgres>;

#[derive(Debug, Clone )]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub enable_wal_mode: bool,
    pub enable_foreign_keys: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self{
            database_url : "postgresql://crawler_user:crawler_pass@localhost:5432/crawler_db".to_string(),
            max_connections: 10,
            enable_wal_mode: true,
            enable_foreign_keys: true,
        }
    }
}

pub struct Database;

impl Database {
    // create a new db connection pool
    pub async fn connect(config: &DatabaseConfig) -> Result<DatabasePool> {
        info!("Connecting to database : {}", config.database_url);

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.database_url)
            .await?;

        info!("Database connected successfully");
        Ok(pool)
    }

    // Run database migrations
    pub async fn migrate(pool: &DatabasePool) -> Result<()> {
        info!("Running migrations ...");

        // Read and execute initial schema
        let initial_schema = include_str!("../../migrations/001_initial.sql");

        // Execute the schema (Sqlite can handle multiple statements)
        let mut tx = pool.begin().await?;

        // split by semicolon and execute each statement
        for statement in initial_schema.split(";") {
            let statement = statement.trim();
            if !statement.is_empty() && !statement.starts_with("__"){
                sqlx::query(statement).execute(&mut *tx).await.map_err(|e|{
                    error!("Failed to run migration: {}", statement);
                    e
                })?;
            }
        }
        tx.commit().await?;

        // Create performance indexes
        Self::create_indexes(pool).await?;

        // Add pagerank column migration
        Self.migrate_pagerank(pool).await?;

        info!("Database migration complete");

        Ok(())
    }


    // create performance indexes
    async fn create_indexes(pool: &DatabasePool) -> Result<()> {
        info!("Creating indexes ...");

        let indexes = vec![
            "CREATE INDEX IF NOT EXISTS idx_pages_domain ON pages(domain);",
            "CREATE INDEX IF NOT EXISTS idx_pages_quality ON pages(quality_score DESC);",
            "CREATE INDEX IF NOT EXISTS idx_pages_crawled_at ON pages(crawled_at DESC);",
            "CREATE INDEX IF NOT EXISTS idx_pages_url_hash ON pages(url_hash);",
            "CREATE INDEX IF NOT EXISTS idx_pages_content_hash ON pages(content_hash);",
            "CREATE INDEX IF NOT EXISTS idx_pages_status_code ON pages(status_code);",
            "CREATE INDEX IF NOT EXISTS idx_links_source ON links(source_page_id);",
            "CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_page_id);",
            "CREATE INDEX IF NOT EXISTS idx_links_target_url ON links(target_url);",
            "CREATE INDEX IF NOT EXISTS idx_sessions_started ON crawl_sessions(started_at DESC);",
            "CREATE INDEX IF NOT EXISTS idx_sessions_status ON crawl_sessions(status);",
            "CREATE INDEX IF NOT EXISTS idx_domains_last_crawled ON domains(last_crawled DESC);",
        ];

        for index_sql in indexes {
            sqlx::query(index_sql).execute(pool).await?;
        }

        info!("Database indexes created successfully");
        Ok(())
    }

    // check db health
    pub async fn health_check(pool: &DatabasePool) -> bool{
        match sqlx::query("SELECT 1 as health_check").fetch_one(pool).await{
            Ok(row)=>{
                let result: i32 = row.get("health_check");
                result == 1
            }
            Err(e)=>{
                warn!("Database health check failed, {}", e);
                false
            }
        }
    }

    // Get database statistics
    pub async fn get_database_stats(pool: &DatabasePool) -> Result<crate::storage::DatabaseStats>{
        let row = sqlx::query(r#"
                SELECT
                    (SELECT COUNT(*) FROM pages) as total_pages,
                    (SELECT COUNT(*) FROM links) as total_links,
                    (SELECT COUNT(DISTINCT domain) FROM pages) as total_domains,
                    (SELECT AVG(quality_score) FROM pages WHERE quality_score > 0) as avg_quality_score,
                    (SELECT COUNT(*) FROM crawl_sessions) as crawl_sessions
                "#)
            .fetch_one(pool)
        .await?;

        // caclculate databse size (approximate for SQLite)
        let size_mb = Self::calculate_database_size(pool).await.unwrap_or(0.0);

        Ok(crate::storage::DatabaseStats {
            total_pages: row.get("total_pages"),
            total_links: row.get("total_links"),
            total_domains: row.get("total_domains"),
            avg_quality_score: row.get("avg_quality_score"),
            crawl_sessions: row.get("crawl_sessions"),
            database_size_mb: 0.0,
        })
    }

    // calculate approximate db size
    async fn calculate_database_size(pool: &DatabasePool) -> Result<f64>{
        let row = sqlx::query("PRAGMA page_count; PRAGMA page_size;")
        .fetch_one(pool)
        .await?;

        // This is a simplified calculation - actual implementation would be more complex
        Ok(0.0) // Placeholder - would calculate from page_count * page_size
    }

    async fn migrate_pagerank(&self, pool: &DatabasePool) ->Result<()> {
        info!("Adding PageRank column...");

        // Add pagerank column if it doesn't exist
        sqlx::query(
            "ALTER TABLE pages ADD COLUMN IF NOT EXISTS pagerank REAL DEFAULT 0.0"
        )
            .execute(pool)
            .await?;

        // Create index for PageRank
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_pagerank ON pages(pagerank DESC)"
        )
            .execute(pool)
            .await?;

        info!("PageRank column added successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_database_connection() {
        let config = DatabaseConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            enable_wal_mode: false, // Disable WAL for in-memory
            enable_foreign_keys: true,
        };

        let pool = Database::connect(&config).await.unwrap();
        assert!(Database::health_check(&pool).await);
    }

    #[tokio::test]
    async fn test_database_migrations() {
        let config = DatabaseConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            enable_wal_mode: false,
            enable_foreign_keys: true,
        };

        let pool = Database::connect(&config).await.unwrap();
        let result = Database::migrate(&pool).await;
        assert!(result.is_ok());

        // Verify tables were created
        let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(count >= 4); // pages, links, crawl_sessions, domains
    }

    #[tokio::test]
    async fn test_database_stats() {
        let config = DatabaseConfig {
            database_url: "sqlite::memory:".to_string(),
            max_connections: 5,
            enable_wal_mode: false,
            enable_foreign_keys: true,
        };

        let pool = Database::connect(&config).await.unwrap();
        Database::migrate(&pool).await.unwrap();

        let stats = Database::get_database_stats(&pool).await.unwrap();
        assert_eq!(stats.total_pages, 0);
        assert_eq!(stats.total_links, 0);
        assert_eq!(stats.total_domains, 0);
    }
}