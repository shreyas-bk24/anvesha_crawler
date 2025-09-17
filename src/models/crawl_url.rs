use serde::{Serialize, Deserialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrawlUrl{
    pub url: String,
    pub priority : f64,
    pub depth : u64,
    pub discovered_at : u64
}

impl PartialEq for CrawlUrl {
    fn eq(&self, other: &Self) -> bool {
        self.priority.partial_cmp(&other.priority) == Some(Ordering::Equal)
    }
}

impl Eq for CrawlUrl {}

impl PartialOrd for CrawlUrl {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for CrawlUrl{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}


