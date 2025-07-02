
use chiral_client::{create_client, get_credit_points};
mod common;

#[tokio::test]
async fn credit_integration_test(){
    let url = common::get_url();
    let mut client = create_client(&url).await.expect("Failed to create client");
    let email = common::get_user_email();
    let token_auth = common::get_token_auth();
    let points_value = get_credit_points(&mut client, &email, &token_auth).await.expect("Failed to get credit points. Check network, API URL, and credentials.");
    assert_eq!(points_value.as_f64().expect("Credit points received are not an f64 number."),7980.002);
}
