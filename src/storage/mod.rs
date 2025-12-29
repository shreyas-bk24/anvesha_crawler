// storage module for persistant data management


pub mod database;
pub mod models;
pub mod repository;
pub mod search_index;
pub mod cache;
pub mod export;
mod tests;
// Re-export main types

pub use models::{StoredPage, SearchResult, DatabaseStats};


// storage errors
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Search index error: {0}")]
    SearchIndex(String),
    #[error("Cache error: {0}")]
    Cache(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Export error: {0}")]
    Export(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;