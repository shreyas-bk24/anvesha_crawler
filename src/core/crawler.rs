use std::cmp::Ordering;
use crate::config::CrawlerConfig;
use crate::core::{UrlFrontier, PageProcessor};
pub(crate) use crate::models::{CrawlUrl, PageData, CrawlStatistics};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering}; // Fixed import
use tokio::task::JoinSet;
use tracing::{error, info};
use crate::core::scheduler::CrawlScheduler;

/// Main web crawler that orchestrates the crawling process
#[derive(Clone)] // Add Clone derive
pub struct WebCrawler {
    config: CrawlerConfig,
    url_frontier: Arc<UrlFrontier>, // Wrap in Arc for sharing
    page_processor: Arc<PageProcessor>, // Wrap in Arc for sharing
    scheduler: Arc<CrawlScheduler>, // Fixed type name and wrap in Arc

    // Statistics tracking
    pages_crawled: Arc<AtomicUsize>, // Fixed field name and type
    pages_failed: Arc<AtomicUsize>,
    start_time: std::time::Instant,
}


impl WebCrawler {
    pub async fn new(config: CrawlerConfig) -> crate::Result<Self> {
        let url_frontier = Arc::new(UrlFrontier::new(config.crawler.max_pages * 10));
        let mut page_processor = PageProcessor::new();

        // Add priority domains from config
        for domain in &config.algorithms.priority_boost_domains {
            page_processor.add_priority_domain(domain.clone());
        }

        let scheduler = Arc::new(CrawlScheduler::new(&config));

        let crawler = Self {
            config,
            url_frontier,
            page_processor: Arc::new(page_processor),
            scheduler,
            pages_crawled: Arc::new(AtomicUsize::new(0)), // Fixed field name
            pages_failed: Arc::new(AtomicUsize::new(0)),
            start_time: std::time::Instant::now(),
        };

        Ok(crawler)
    }

    /// Start the crawling process
    pub async fn start_crawling(&self) -> crate::Result<CrawlStatistics> {
        info!("Starting web crawler with {} seed URLs", self.config.crawler.seed_urls.len());

        // Add seed URLs to frontier
        self.initialize_frontier().await?;

        // Start crawling workers
        let mut join_set = JoinSet::new();

        // Spawn crawler worker tasks
        for worker_id in 0..self.config.crawler.concurrent_requests { // Fixed field name
            let crawler_clone = self.clone();
            join_set.spawn(async move {
                crawler_clone.crawler_worker(worker_id).await
            });
        }

        // Wait for all workers to complete
        while let Some(result) = join_set.join_next().await { // Fixed method name
            if let Err(e) = result {
                error!("Web crawler worker failed: {}", e);
            }
        }

        // Generate final stats
        let stats = self.generate_statistics().await;
        info!("Crawling completed: {:?}", stats);

        Ok(stats)
    }

    /// Individual crawler worker
    async fn crawler_worker(&self, worker_id: usize) -> crate::Result<()> { // Remove mut self
        info!("Starting crawler worker {}", worker_id);

        while self.pages_crawled.load(AtomicOrdering::Relaxed) < self.config.crawler.max_pages {
            // Get next URL from frontier
            let crawl_url = match self.url_frontier.next_url().await {
                Some(url) => url,
                None => {
                    // No more URLs, check if we should wait or exit
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    if self.url_frontier.is_empty().await {
                        break;
                    }
                    continue;
                }
            };

            // Skip if already crawled
            if self.url_frontier.is_crawled(&crawl_url.url) {
                continue;
            }

            // Extract domain for rate limiting
            let domain = self.extract_domain(&crawl_url.url)?; // Fixed method name

            // Crawl the page
            match self.crawl_single_page(crawl_url, &domain).await {
                Ok(_) => {
                    self.pages_crawled.fetch_add(1, AtomicOrdering::Relaxed); // Fixed increment logic
                }
                Err(e) => {
                    self.pages_failed.fetch_add(1, AtomicOrdering::Relaxed);
                    error!("Failed to crawl page: {}", e);
                }
            }
        }

        info!("Crawler worker {} finished", worker_id);
        Ok(())
    }

    /// Crawl a single page
    async fn crawl_single_page(&self, crawl_url: CrawlUrl, domain: &str) -> crate::Result<()> {
        let url = crawl_url.url.clone();

        // Use scheduler to manage the request
        let page_data = self.scheduler.schedule_crawl(domain, || async {
            // This is where you'd integrate with network module
            // For now, this is a placeholder
            self.fetch_and_process_page(crawl_url.clone()).await
        }).await?;

        // Mark as crawled
        self.url_frontier.mark_crawled(&url);

        // Add discovered links to frontier
        let links_added = self.url_frontier.add_urls(page_data.outgoing_links).await;

        info!("Crawled: {} (found {} new links)", url, links_added);

        // Here you'd save the page data to storage
        // self.save_page_data(page_data).await?;

        Ok(())
    }

    /// Fetch and process a single page (placeholder)
    async fn fetch_and_process_page(&self, crawl_url: CrawlUrl) -> Result<PageData, Box<dyn std::error::Error + Send + Sync>> {
        // This is a placeholder - you'll implement actual HTTP fetching in the network module
        // For now, return mock data
        Ok(PageData {
            url: crawl_url.url,
            title: Some("Mock Title".to_string()),
            description: None,
            keywords: vec![],
            content: "Mock content".to_string(),
            outgoing_links: vec![],
            word_count: 2,
            content_quality_score: 0.5,
            crawled_at: chrono::Utc::now(),
            depth: crawl_url.depth as u32,
        })
    }

    /// Initialize the URL frontier with seed URLs
    async fn initialize_frontier(&self) -> crate::Result<()> {
        for seed_url in &self.config.crawler.seed_urls {
            let crawl_url = CrawlUrl {
                url: seed_url.clone(),
                priority: 10.0, // High priority for seed URLs
                depth: 0,
                discovered_at: chrono::Utc::now().timestamp() as u64,
            };

            self.url_frontier.add_url(crawl_url).await;
        }

        info!("Initialized frontier with {} seed URLs", self.config.crawler.seed_urls.len());
        Ok(())
    }

    /// Extract domain from URL for rate limiting
    fn extract_domain(&self, url: &str) -> crate::Result<String> {
        let parsed_url = url::Url::parse(url)?;
        Ok(parsed_url.host_str().unwrap_or("unknown").to_string())
    }

    /// Generate crawling statistics
    async fn generate_statistics(&self) -> CrawlStatistics {
        let frontier_stats = self.url_frontier.get_stats().await;
        let scheduler_stats = self.scheduler.get_stats();

        CrawlStatistics {
            pages_crawled: self.pages_crawled.load(AtomicOrdering::Relaxed),
            pages_failed: self.pages_failed.load(AtomicOrdering::Relaxed),
            urls_discovered: frontier_stats.seen_count,
            urls_in_queue: frontier_stats.queue_size,
            elapsed_time: self.start_time.elapsed(),
            crawl_rate: self.pages_crawled.load(AtomicOrdering::Relaxed) as f64 / self.start_time.elapsed().as_secs_f64(),
        }
    }
}
