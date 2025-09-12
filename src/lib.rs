//! Search Engine Crawler Library
//!
//! A modular, high-performance web crawler designed for search engines.

pub mod config;
pub mod core;
pub mod algorithms;
pub mod network;
pub mod parsing;
pub mod storage;
pub mod utils;
pub mod models;
pub mod api;

// Re-export commonly used types
pub use config::CrawlerConfig;
pub use core::crawler::WebCrawler;
pub use models::{CrawlUrl, PageData, CrawlResult};

/// Main crawler error type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Initialize the crawler with logging and metrics
pub async fn init() -> Result<()> {
    utils::logging::init_logging()?;
    utils::metrics::init_metrics().await?;
    Ok(())
}
