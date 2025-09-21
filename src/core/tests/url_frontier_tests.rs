use crate::core::UrlFrontier;
use crate::models::CrawlUrl;

#[tokio::test]
async fn test_url_frontier_basic_operations() {
    let frontier = UrlFrontier::new(100);

    // Test adding URLs
    let url1 = CrawlUrl {
        url: "https://example.com".to_string(),
        priority: 10.0,
        depth: 0,
        discovered_at: chrono::Utc::now().timestamp() as u64,
    };

    let url2 = CrawlUrl {
        url: "https://test.com".to_string(),
        priority: 5.0,
        depth: 1,
        discovered_at: chrono::Utc::now().timestamp() as u64,
    };

    assert!(frontier.add_url(url1.clone()).await);
    assert!(frontier.add_url(url2.clone()).await);

    // Test deduplication
    assert!(!frontier.add_url(url1.clone()).await); // Should not add duplicate

    // Test priority ordering (higher priority first)
    let next_url = frontier.next_url().await.unwrap();
    assert_eq!(next_url.priority, 10.0);
    assert_eq!(next_url.url, "https://example.com");

    let next_url = frontier.next_url().await.unwrap();
    assert_eq!(next_url.priority, 5.0);
    assert_eq!(next_url.url, "https://test.com");

    // Test empty frontier
    assert!(frontier.next_url().await.is_none());
}

#[tokio::test]
async fn test_url_frontier_crawled_tracking() {
    let frontier = UrlFrontier::new(100);

    let url = "https://example.com";

    // Initially not crawled
    assert!(!frontier.is_crawled(url));

    // Mark as crawled
    frontier.mark_crawled(url);
    assert!(frontier.is_crawled(url));

    // Test stats
    let stats = frontier.get_stats().await;
    assert_eq!(stats.crawled_count, 1);
}

#[tokio::test]
async fn test_url_frontier_capacity_limits() {
    let frontier = UrlFrontier::new(2); // Small capacity for testing

    let url1 = CrawlUrl {
        url: "https://example.com/1".to_string(),
        priority: 1.0,
        depth: 0,
        discovered_at: chrono::Utc::now().timestamp() as u64,
    };

    let url2 = CrawlUrl {
        url: "https://example.com/2".to_string(),
        priority: 1.0,
        depth: 0,
        discovered_at: chrono::Utc::now().timestamp() as u64,
    };

    let url3 = CrawlUrl {
        url: "https://example.com/3".to_string(),
        priority: 1.0,
        depth: 0,
        discovered_at: chrono::Utc::now().timestamp() as u64,
    };

    // Add URLs up to capacity
    assert!(frontier.add_url(url1).await);
    assert!(frontier.add_url(url2).await);

    // Should reject when at capacity
    assert!(!frontier.add_url(url3).await);
}
