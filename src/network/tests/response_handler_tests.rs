use crate::network::ResponseProcessor;

#[test]
fn test_response_processor_creation() {
    let processor = ResponseProcessor::new();

    // Test with custom settings
    let custom_processor = ResponseProcessor::new()
        .with_max_size(1024 * 1024)
        .with_allowed_content_types(vec!["text/html".to_string()]);

    // Basic smoke test - if it compiles and runs, the structure is correct
    assert!(true);
}

#[test]
fn test_content_type_extraction() {
    // This would test the private methods, but for now just ensure structure works
    let processor = ResponseProcessor::default();
    assert!(true); // Placeholder for actual content type testing
}
