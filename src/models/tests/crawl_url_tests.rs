use crate::models::CrawlUrl;
use std::cmp::Ordering;

#[test]
fn test_crawl_url_priority_ordering() {
    let url1 = CrawlUrl {
        url: "https://example.com".to_string(),
        priority: 10.0,
        depth: 0,
        discovered_at: 1000,
    };

    let url2 = CrawlUrl {
        url: "https://test.com".to_string(),
        priority: 5.0,
        depth: 1,
        discovered_at: 2000,
    };

    // Higher priority should come first
    assert_eq!(url1.cmp(&url2), Ordering::Greater);
    assert_eq!(url2.cmp(&url1), Ordering::Less);
}

#[test]
fn test_crawl_url_creation() {
    let url = CrawlUrl {
        url: "https://example.com/test".to_string(),
        priority: 7.5,
        depth: 2,
        discovered_at: chrono::Utc::now().timestamp() as u64,
    };

    assert_eq!(url.url, "https://example.com/test");
    assert_eq!(url.priority, 7.5);
    assert_eq!(url.depth, 2);
    assert!(url.discovered_at > 0);
}
