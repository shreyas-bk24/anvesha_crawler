use crate::config::CrawlerConfig;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::{ sleep, Duration, Instant};
use tracing::{debug, warn, info };

// manage crawling scheduling and rate limiting
pub struct CrawlScheduler {
    // semaphore to limit incorrect req
    semaphore: Arc<Semaphore>,

    // delay between request to the same domain
    request_delay : Duration,

    // pre-domain rate limiting
    domain_delays : dashmap::DashMap<String, Instant>,

    // maximum number of retries for failed request
    max_retries: u32,
}

impl CrawlScheduler {
    pub fn new(config: &CrawlerConfig) -> Self{
        Self{
            semaphore: Arc::new(Semaphore::new(config.crawler.concorrent_requests)),
            request_delay: Duration::from_millis(config.network.request_delay_ms),
            domain_delays: dashmap::DashMap::new(),
            max_retries: config.network.max_retries,
        }
    }

    pub async fn acquire_permit(&self) -> tokio::sync::SemaphorePermit<'_>{
        self .semaphore
            .aquire()
            .await
            .expect("Semaphore should not be closed")
    }

    // check if we should delay before crawling this domain
    pub async fn respect_domain_delay(&self, domain: &str){
        if let Some(last_request_time) = self.domain_delays.get(domain){
            let elapsed = last_request_time.elapsed();
            if elapsed < self.request_delay{
                let remaining_delay = self.request_delay - elapsed;
                debug!("Delay {}ms for domain : {}", remaining_delay.as_millis(), domain);
                sleep(remaining_delay).await;
            }
        }
        // update last request time for this domain
        self.domain_delays.insert(domain.to_string(), Instant::now());
    }

    // Execute a crawling task with proper scheduling
    pub async fn schedule_crawl<F, Fut, T>(&self, domain: &str, task: F)-> Result<T, SchedulerError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
    {
        // Acquire semaphore permit (limits concurrency)
        let _permit = self.acquire_permit().await;

        // Respect domain-specific delays
        self.resepct_domain_delay(domain).await;

        // Execute the task with retry logic
        let mut attempts = 0;

        loop {
            attempts += 1;

            match task().await{
                Ok(result) => return Ok(result),
                Err(e)=>{
                    if attempts > self.max_retries{
                        return Err(SchedulerError::MaxRetriesExceeded(e.to_string()));
                    }

                    let delay = Duration::from_millis(1000*attempts as u64);
                    warn!("Request failed (attempt {}, retrying in {}ms: {})", attempts, self.request_delay.as_millis(), e);
                    sleep(delay).await;
                }
            }
        }
    }

    // get current scheduler stats
    pub fn get_stats(&self) -> SchedulerStats {
        SchedulerStats {
            available_permits : self.semaphore.available_permits(),
            active_domains : self.domain_delays.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub available_permits : usize,
    pub active_domains : usize,
}

#[derive(Debug, thiserror::Error)]
pub enum SchedulerError {
    #[error("MaxRetries exceeded: {0}")]
    MaxRetriesExceeded(String),
}