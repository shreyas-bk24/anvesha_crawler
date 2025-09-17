use crate::config::CrawlerConfig;
use crate::core::{UrlFrontier, PageProcessor, CrawlScheduler};
use crate::models::{CrawlUrl, PageData, CrawlStatistics, };
use std::sync::Arc;
use std::sync::automatic::{Automatic, Ordering};
use tokio::task::JoinSet;
use tracing::{error, info, warn};

// Main web crawler that orchestrates the crawling process

pub struct WebCrawler{
    config: CrawlerConfig,
    url_frontier: UrlFrontier,
    page_processor: PageProcessor,
    scheduler: Scheduler,

    // statistic tracking
    page_crawled : Arc<AutomaticUsize>,
    pages_failed: Arc<AutomaticUsize>,
    start_time : std::time::Instant,
}

impl WebCrawler {
    pub async fn new(config : CrawlerConfig) -> crate::Result<Self> {
        let url_frontier = UrlFrontier::new(config.crawler.max_pages * 10);
        let mut page_processor = PageProcessor::new();

        // Add priority domains from config
        for domain in &config.algorithms.priority_boost_domains{
            page_processor.add_priority_domain(domain.clone());
        }

        let scheduler = CrawlScheduler::new(&config);

        let crawler = Self{
            config,
            url_frontier,
            page_processor,
            scheduler,
            page_crawled: Arc::new(AutomaticUsize::new(0)),
            pages_failed: Arc::new(AutomaticUsize::new(0)),
            start_time: std::time::Instant::now(),
        };

        Ok(crawler)
    }

    // start the crawling process
    pub async fn start_crawling(&self) -> crate::Result<CrawlStatistics>{
        info!("Start web crawler with {} seed URLs", self.config.crawler.seed_urls.len());
        
        // add seed urls to a frontier
        self.initialize_frontier().await?;
        
        // start crawling workers
        let mut join_set = JoinSet::new();
        
        // spawn crawler worker tasks
        for worker_id in 0..self.config.crawler_requests{
            let crawler_clone = self.clone();
            join_set.spawn(async move { 
                crawler_clone.crawler_worker(worker_id).await;
            });
        }
        
        // wait for all workers to complete
        while let Some(result) = join_set.next().await {
            if let Err(e) = result{
                error!("Web crawler worker failed: {}", e);
            }
        }
        
        // generate final stats
        let stats = self.generate_statistics().await;
        info!("Crawling completed : {:?}",stats);
        
        Ok(stats)
    }
    
    // individual crawl worker
    async fn crawler_worker(&mut self, worker_id: usize) -> crate::Result<()> {
        info!("Start crawling worker {}", worker_id);

        while  self.page_crawled.load(Ordering::Relaxed) < self.config.crawler.max_pages {
            // Get the next url from a frontier
            let crawler_url = match self.url_frontier.next_url().await{
                Some(url) => url,
                None =>{
                    // No more urls, check if we should wait or exit
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    if self.url_frontier.is_empty().await(){
                        break
                    }
                    continue;
                }
            };

            // skip if already crawled
            if self.url_frontier.is_crawled(&crawler_url.url){
                continue;
            }

            // Extract domain for the rate limiting
            let domain = self.extracted_domain(&crawler_url.url)?;

            // crawl the page
            match self.crawl_single_page(crawl_url, &domain).await{
                Ok(_)=>{
                    self.pages_failed.fetch_add(1, Ordering::Relaxed);
                }
                Err(e)=>{
                    self.pages_failed.fetch_add(1, Ordering::Relaxed);
                    error!("Failed to crawl page : {}",e);
                }
            }
        }
        info!("Crawling completed : {:?}",worker_id);
        Ok(())
    }

    // Crawl single page
    async fn crawl_single_page(&self, crawl_url : CrawlUrl, domain: &str) -> crate::Result<()>{
        let url = crawl_url.url.clone();

        // use scheduler to manage the request
        let page_data = self.scheduler.schedule_crawl(domain, || async {
            // this is where you would integrate nw module
            // for now it is empty
            self.fetch_and_process_page(crawl_url).await
        }).await?;

        // mark as crawled
        self.url_frontier.mark_crawled(&url);

        // Add discovered links to the frontier
        let links_added = self.url_frontier.add_urls(page_data.outgoing_links).await?;

        info!("Crawled : {} (found {} new links)",url, links_added);

        // here you would save the page data to storage
        // self.save_page_data(page_data).await?;

        Ok(())
    }

    /// Fetch and process a single page (placeholder - you'll implement this with network module)
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
            depth: crawl_url.depth,
        })
    }/// Initialize the URL frontier with seed URLs
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
            pages_crawled: self.pages_crawled.load(Ordering::Relaxed),
            pages_failed: self.pages_failed.load(Ordering::Relaxed),
            urls_discovered: frontier_stats.seen_count,
            urls_in_queue: frontier_stats.queue_size,
            elapsed_time: self.start_time.elapsed(),
            crawl_rate: self.pages_crawled.load(Ordering::Relaxed) as f64 / self.start_time.elapsed().as_secs_f64(),
        }
    }
}

// You'll need to implement Clone for WebCrawler
impl Clone for WebCrawler {
    fn clone(&self) -> Self {
        // Implementation depends on your specific needs
        // This is a simplified version
        unimplemented!("Implement Clone for WebCrawler based on your Arc/Rc usage")
    }
}