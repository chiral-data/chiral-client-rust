
use chiral_client::create_client;
mod common;

#[tokio::test]
async fn client_integration_test() {
    let url = common::get_url();
    let result = create_client(&url).await;
    assert!(result.is_ok(), "Client creation failed: {:?}", result.err());
}
