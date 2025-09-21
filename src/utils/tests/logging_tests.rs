use crate::utils;

#[test]
fn test_logging_initialization() {
    // Test that logging can be initialized without panicking
    let result = utils::init_logging();

    // First call should succeed
    assert!(result.is_ok());

    // Second call should also be OK (idempotent)
    let result2 = utils::init_logging();
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_metrics_initialization() {
    let result = utils::init_metrics().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_full_initialization() {
    let result = utils::init().await;
    assert!(result.is_ok());
}
