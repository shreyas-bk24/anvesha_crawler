#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Database;
    use crate::models::PageData;
    use crate::models::PageData;
    use chrono::Utc;

    async fn setup_test_db() -> PageRepository {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
        Database::migrate(&pool).await.unwrap();
        PageRepository::new(pool)
    }

    #[tokio::test]
    async fn test_save_and_get_page() {
        let repo = setup_test_db().await;

        let page_data = PageData {
            url: "https://example.com/test".to_string(),
            title: Some("Test Page".to_string()),
            description: Some("A test page".to_string()),
            keywords: vec!["test".to_string()],
            content: "Test content for the page".to_string(),
            outgoing_links: vec![],
            word_count: 5,
            content_quality_score: 0.8,
            crawled_at: Utc::now(),
            depth: 1,
        };

        // Save page
        let page_id = repo.save_page(&page_data, 1).await.unwrap();
        assert!(page_id > 0);

        // Get page by ID
        let stored_page = repo.get_page_by_id(page_id).await.unwrap();
        assert!(stored_page.is_some());
        let stored_page = stored_page.unwrap();
        assert_eq!(stored_page.url, page_data.url);
        assert_eq!(stored_page.title, page_data.title);

        // Get page by URL
        let page_by_url = repo.get_page_by_url(&page_data.url).await.unwrap();
        assert!(page_by_url.is_some());
        assert_eq!(page_by_url.unwrap().id, page_id);

        // Check URL exists
        let exists = repo.url_exists(&page_data.url).await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    async fn test_search_pages() {
        let repo = setup_test_db().await;

        let page1 = PageData {
            url: "https://example.com/rust".to_string(),
            title: Some("Rust Programming".to_string()),
            description: Some("Learn Rust programming language".to_string()),
            keywords: vec!["rust".to_string(), "programming".to_string()],
            content: "Rust is a systems programming language".to_string(),
            outgoing_links: vec![],
            word_count: 7,
            content_quality_score: 0.9,
            crawled_at: Utc::now(),
            depth: 1,
        };

        let page2 = PageData {
            url: "https://example.com/python".to_string(),
            title: Some("Python Programming".to_string()),
            description: Some("Learn Python programming language".to_string()),
            keywords: vec!["python".to_string(), "programming".to_string()],
            content: "Python is a high-level programming language".to_string(),
            outgoing_links: vec![],
            word_count: 7,
            content_quality_score: 0.8,
            crawled_at: Utc::now(),
            depth: 1,
        };

        // Save pages
        repo.save_page(&page1, 1).await.unwrap();
        repo.save_page(&page2, 1).await.unwrap();

        // Search for "rust"
        let results = repo.search_pages("rust", 10).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, Some("Rust Programming".to_string()));

        // Search for "programming"
        let results = repo.search_pages("programming", 10).await.unwrap();
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_page_filter() {
        let repo = setup_test_db().await;

        let page = PageData {
            url: "https://github.com/test".to_string(),
            title: Some("GitHub Test".to_string()),
            description: None,
            keywords: vec![],
            content: "GitHub repository content".to_string(),
            outgoing_links: vec![],
            word_count: 3,
            content_quality_score: 0.7,
            crawled_at: Utc::now(),
            depth: 1,
        };

        repo.save_page(&page, 1).await.unwrap();

        // Filter by domain
        let filter = PageFilter::new().with_domain("github.com".to_string());
        let results = repo.get_pages(&filter).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].domain, "github.com");

        // Filter by quality
        let filter = PageFilter::new().with_min_quality(0.8);
        let results = repo.get_pages(&filter).await.unwrap();
        assert_eq!(results.len(), 0); // Quality is 0.7, below threshold

        let filter = PageFilter::new().with_min_quality(0.6);
        let results = repo.get_pages(&filter).await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[tokio::test]
    async fn test_crawl_session() {
        let repo = setup_test_db().await;
        let config = crate::config::CrawlerConfig::default();
        let seed_urls = vec!["https://example.com".to_string()];

        // Create session
        let session_id = repo.create_crawl_session(&seed_urls, &config).await.unwrap();
        assert!(session_id > 0);

        // Update session
        repo.update_crawl_session(session_id, 5, 1).await.unwrap();

        // Complete session
        repo.complete_crawl_session(session_id, "completed").await.unwrap();

        // Verify stats
        let stats = repo.get_stats().await.unwrap();
        assert_eq!(stats.crawl_sessions, 1);
    }
}
