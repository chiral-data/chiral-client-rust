use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};
use std::str::FromStr;
use serde_json::json;
use crate::api::client::chiral::chiral_client::ChiralClient;
use crate::api::client::chiral::RequestUserCommunicate;


pub mod chiral {
    tonic::include_proto!("chiral"); 
}




pub async fn submit_test_job(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, job_type_name: &str, index: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let end_point = "SubmitTestJob";
    
    let payload = json!({
        end_point: [job_type_name, index]
    });
    
    let serialized = serde_json::to_string(&payload)?;

    let req_msg = RequestUserCommunicate {
        serialized_request: serialized,
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

pub async fn get_jobs(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, offset: u32, count_per_page: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {    let end_point = "GetJobs";
    let _end_point = "GetJobs";
    let serialized = format!(
        "{{\"{}\": [{}, {}]}}",
        end_point, offset, count_per_page
    );

    let req_msg = RequestUserCommunicate {
        serialized_request: serialized.clone(),
    };

    println!("Sending payload: {}", serialized); 

    let mut request = Request::new(req_msg);
    request.metadata_mut().insert("user_id", MetadataValue::from_str(email)?);
    request.metadata_mut().insert("auth_token", MetadataValue::from_str(token_auth)?);

    let response = client.user_communicate(request).await?.into_inner();
    println!("Reply JSON: {}", response.serialized_reply);


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

pub async fn get_job(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str,job_id: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "GetJob";
    let serialized = format!(
    "{{\"{}\": \"{}\"}}",
    end_point, job_id
    );

    let req_msg = RequestUserCommunicate{
        serialized_request: serialized.clone(),
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
pub async fn submit_job(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, command_string: &str, project_name: &str, input_files: &[&str], output_files: &[&str]) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let end_point = "SubmitJob";

    let input_files_json = serde_json::to_string(&input_files)?;
    let output_files_json = serde_json::to_string(&output_files)?;

    let serialized = format!(
        "{{\"{}\": [\"{}\", \"{}\", {}, {}]}}",
        end_point, command_string, project_name, input_files_json, output_files_json
    );

    let req_msg = RequestUserCommunicate {
        serialized_request: serialized.clone(),
    };

    let mut request = Request::new(req_msg);
    request.metadata_mut().insert("user_id", MetadataValue::from_str(email)?);
    request.metadata_mut().insert("auth_token", MetadataValue::from_str(token_auth)?);

    let response = client.user_communicate(request).await?.into_inner();
    println!("Reply JSON: {}", response.serialized_reply);

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