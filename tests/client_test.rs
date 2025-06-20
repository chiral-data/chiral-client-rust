use chiral_client_rust::create_client;
mod common;

#[tokio::test]
async fn test_create_client_integration() {
    let url = common::get_url();
    let result = create_client(&url).await;
    assert!(result.is_ok(), "Client creation failed: {:?}", result.err());
}
