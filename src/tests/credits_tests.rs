use crate::api::get_credit_points;
use crate::api::create_client;

#[cfg(test)]
mod tests{
    use super::{get_credit_points, create_client};
    #[tokio::test]
    async fn test_get_credit_points(){
        dotenvy::from_filename(".env").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("CHIRAL_STAGING_API_URL environment variable not set");
        let email = std::env::var("TEST_EMAIL").expect("TEST_EMAIL environment variable not set");
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("TEST_TOKEN_AUTH environment variable not set");
        let mut client = create_client(&url).await.expect("Failed to create API client. Ensure CHIRAL_STAGING_API_URL is valid and the client can be created.");
        let points_value = get_credit_points(&mut client, &email, &token_auth).await.expect("Failed to get credit points. Check network, API URL, and credentials.");
        assert_eq!(points_value.as_f64().expect("Credit points received are not an i64 number."),3000.00);
    }
}