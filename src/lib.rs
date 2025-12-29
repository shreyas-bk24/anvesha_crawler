//! Search Engine Crawler Library

pub mod config;
pub mod core;
pub mod models;
pub mod utils;
pub mod network;
pub mod storage;
pub mod search;
pub mod algorithms;

use chrono::offset;
// Re-export commonly used types
pub use config::CrawlerConfig;
use tantivy::snippet;
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

use std::path::Path;
use crate::search::query::SearchQuery;
use crate::search::filters::{self, SearchFilter, SortBy};

// public search engine interface for adapters and integrations
pub struct SearchEngine{
    inner: SearchQuery,
}

impl  SearchEngine {
    // initialize search engine interface for adapters and integrations
    pub fn new(index_path: &Path) -> Result<Self>{
        let inner = SearchQuery::new(index_path)?;
        Ok(Self { inner })
    }
    // execute search query

    pub fn search(
        &self,
        query: &str,
        limit: usize,
        offset: usize,
        filters: SearchFilter,
        sort: SortBy,
        snippets: bool,
        highlight: bool,
    ) -> Result<Vec<crate::search::SearchResult>>{
        let result = self.inner.search_with_filters(query, limit, filters, sort, offset, snippets, highlight,)?;
        Ok(result)
    }
}