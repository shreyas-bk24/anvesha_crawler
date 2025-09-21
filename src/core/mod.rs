//! core crawler components

pub mod crawler;
pub mod url_frontier;
pub mod page_processor;
pub mod scheduler;
mod tests;

pub use url_frontier::UrlFrontier;
pub use page_processor::PageProcessor;
pub use scheduler::CrawlScheduler;