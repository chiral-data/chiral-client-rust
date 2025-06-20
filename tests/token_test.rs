use chiral_client_rust::{create_client, get_token_api, refresh_token_api};
mod common;

#[tokio::test]
async fn token_flow_integration_test() {
    let url = common::get_url();
    let email = common::get_user_email();
    let token = common::get_token_auth();

    let mut client = create_client(&url).await.expect("Failed to create client");

    let token_info = get_token_api(&mut client, &email, &token)
        .await
        .expect("Failed to fetch token API info");

    assert!(
        token_info.is_string(),
        "Expected token info to be a string"
    );
    let token_str = token_info.as_str().expect("Token should be a string");
    println!("Token: {}", token_str);

    let refresh_info = refresh_token_api(&mut client, &email, &token)
        .await
        .expect("Failed to refresh token");

    assert!(
        refresh_info.is_string(),
        "Expected refreshed token info to be a string"
    );
    let refreshed_token = refresh_info.as_str().expect("Refreshed token should be a string");
    println!("Refreshed Token: {}", refreshed_token);

    assert_ne!(token_str, refreshed_token, "Token did not change after refresh");
}
