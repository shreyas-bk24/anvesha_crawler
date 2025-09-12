//! core crawler components

pub mod crawler;
pub mod url_frontier;
pub mod page_processor;
pub mod scheduler;

pub use crawler::Crawler;
pub use url_frontier::UrlFrontier;
pub use page_processor::PageProcessor;
pub use scheduler::Scheduler;