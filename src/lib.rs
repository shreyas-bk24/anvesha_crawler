//! Search Engine Crawler Library

pub mod config;
pub mod core;
pub mod models;
pub mod utils;
pub mod network;
pub mod storage;
pub mod search;
pub mod algorithms;

// Re-export commonly used types
pub use config::CrawlerConfig;
pub use crate::core::crawler::WebCrawler;
pub use models::{CrawlUrl, PageData, CrawlResult, CrawlStatistics};
pub use network::{HttpClient, NetworkError};

/// Main crawler error type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Initialize the crawler with logging and metrics
pub async fn init() -> Result<()> {
    utils::init_logger()?;
    utils::init_metrics().await?;
    Ok(())
}
