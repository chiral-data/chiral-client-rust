use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};
use std::str::FromStr;
use crate::api::client::chiral::chiral_client::ChiralClient;
use crate::api::client::chiral::RequestUserCommunicate;


pub mod chiral {
    tonic::include_proto!("chiral"); 
}


pub async fn get_credit_points(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "GetCreditPoints";
    let serialized = format!(
        "{{\"{}\": null}}",
        end_point
    );

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
