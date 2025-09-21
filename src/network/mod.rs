//! Network module for HTTP requests and response handling

pub mod http_client;
pub mod response_handler;
pub mod error_handler;

// Re-export the main types
pub use http_client::{HttpClient, HttpClientStats};
pub use response_handler::{HttpResponse, ResponseProcessor};
pub use error_handler::{NetworkError, classify_reqwest_error};

// Tests module
#[cfg(test)]
mod tests;
