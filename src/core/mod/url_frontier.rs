//! manages the URL queue with prioritization

use crate::modules::CrawlUrl;
use dashmap::DashSet;
use std::collections::BinaryHeap;
use std::sync::Arc;
use std::tokio::sync::Mutex;
use tracing::{debug, info};

/// thread safe URL frontier that manages crawling queue with prioritization

pub struct UrlFrontier{
     // priority queue for URLs to crawl (higher priority first)
    queue: Arc<Mutex<BinaryHeap<CrawlURL>>>,

    // set of URLs already seen to avoid duplicates
    seen_urls : Arc<DashSet<String>>,

    // max queue size to prevent memory issue
    max_queue_size : usize,
}

impl UrlFrontier {
    pub fn new(max_queue_size: usize) -> Self{
        Self{
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            seen_urls: Arc::new(DashSet::new()),
            crwaled_urls : Arc::new(DashSet::new()),
            max_queue_size,
        }
    }

    // Add URL to a frontier if not already seen
    pub async fn add_url(&self, url: CrawlUrl) -> bool {
        if self.seen_urls.contains(&url.url) {
            return false;
        }

        let mut queue =  self.queue.lock().await;

    //     check queue size limit
        if queue.len() >= self.max_queue_size {
            debug!("url frontier limit exceeded, dropping url {}", url.url);
            return false;
        }

        self.seen_urls.insert(url.url.clone());
        queue.push(url);
        true
    }

//     add multiple URLs at once
    pub async fn add_urls(&self, urls: Vec<CrawlUrl>) -> usize {
        let mut added = 0;
        for url in urls {
            if self.add_url(url).await {
                added += 1;
            }
        }
        added
    }

    // Get the next URL to crawl (the highest priority)

    pub async fn next_url(&self) -> Option<CrawlUrl> {
        let mut queue = self.queue.lock().await;
        queue.pop()
    }

    // mark URL as crawled

    pub fn mark_crawled(&self, url: &str) {
        self.crwaled_urls.insert(url.to_string());
    }

    // check if url is already crawled
    pub fn is_crawled(&self, url: &str) -> bool {
        self.crwaled_urls.contains(url)
    }

    // get queue stats
    pub async fn get_stats(&self) -> FrontierStats {
        let queue_size = self.queue.lock().await.len();
        FrontierStats {
            queue_size,
            seen_count : self.seen_urls.len(),
            crawled_count : self.crwaled_urls.len(),
        }
    }

    // check if the frontier is empty
    pub async fn is_empty(&self) -> bool {
        let queue =  self.queue.lock().await;
        queue.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct FrontierStats {
    queue_size: usize,
    seen_count : usize,
    crawled_count : usize,
}