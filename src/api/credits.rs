use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};
use std::str::FromStr;
use crate::api::client::chiral::chiral_client::ChiralClient;
use crate::api::client::chiral::RequestUserCommunicate;


pub async fn get_credit_points(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "GetCreditPoints";
    let serialized = format!("{{\"{end_point}\": null}}");

    let req_msg = RequestUserCommunicate{
        serialized_request : serialized.clone(),
    }; 
    let mut request = Request::new(req_msg);

    request.metadata_mut().insert("user_id", MetadataValue::from_str(email)?);
    request.metadata_mut().insert("auth_token", MetadataValue::from_str(token_auth)?);

    let response = client.user_communicate(request).await?.into_inner();
    if !response.serialized_reply.trim().is_empty() {
        let parsed: serde_json::Value = serde_json::from_str(&response.serialized_reply)?;
        if let Some(result) = parsed.get(end_point) {
            return Ok(result.clone());
        } else {
            return Err("Missing endpoint data in server response".into());
        }
    }

    if !response.error.trim().is_empty() {
        return Err(format!("Server error: {}", response.error).into());
    }

    Err("Unexpected empty response from server".into())
}

#[cfg(test)]
mod tests{
    use crate::api::{get_credit_points, create_client};
    #[tokio::test]
    async fn test_get_credit_points(){
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("CHIRAL_STAGING_API_URL environment variable not set");
        let email = std::env::var("TEST_EMAIL").expect("TEST_EMAIL environment variable not set");
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("TEST_TOKEN_AUTH environment variable not set");
        let mut client = create_client(&url).await.expect("Failed to create API client. Ensure CHIRAL_STAGING_API_URL is valid and the client can be created.");
        let points_value = get_credit_points(&mut client, &email, &token_auth).await.expect("Failed to get credit points. Check network, API URL, and credentials.");
        assert_eq!(points_value.as_f64().expect("Credit points received are not an f64 number."),7980.002);
    }
}
