// worker/tests/integration_test.rs
// Create this file in a new `tests/` folder

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use std::fs;

#[tokio::test]
async fn test_fetch_and_save_with_mock_api() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Create mock response
    let mock_response = serde_json::json!({
        "schemaVersion": "1.0.0",
        "pairs": [
            {
                "chainId": "sui",
                "priceNative": "0.001234",
                "priceUsd": "0.567890",
                "marketCap": 1234567.89,
                "fdv": 9876543.21
            }
        ]
    });

    // Mount the mock
    Mock::given(method("GET"))
        .and(path("/latest/dex/tokens/0x84604526d71bbe7738c3c02d3c8a48778955718289c03d814d8468b58ae9a898::skelsui::SKELSUI"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
        .mount(&mock_server)
        .await;

    // Update API_URL to point to mock server
    // (You'd need to make API_URL configurable for this)
    
    // Call your function
    // fetch_and_save().await.unwrap();

    // Verify file was created
    assert!(std::path::Path::new("prices.json").exists());

    // Verify content
    let content = fs::read_to_string("prices.json").unwrap();
    assert!(content.contains("sui"));
    assert!(content.contains("0.567890"));

    // Cleanup
    fs::remove_file("prices.json").ok();
}