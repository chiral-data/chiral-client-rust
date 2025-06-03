use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};
use std::str::FromStr;


pub mod chiral {
    tonic::include_proto!("chiral"); 
}

use chiral::chiral_client::ChiralClient;
use chiral::{HelloRequest, RequestUserCommunicate};

async fn create_client(url: &str) -> Result<ChiralClient<Channel>, Box<dyn std::error::Error>> {
    Ok(ChiralClient::connect(url.to_string()).await?)
}

async fn say_hello(client: &mut ChiralClient<Channel>, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(HelloRequest {
        name: name.to_string(),
    });

    let response = client.say_hello(request).await?.into_inner();
    println!("Hello message: {}", response.message);
    Ok(())
}

pub async fn add_user(client: &mut ChiralClient<Channel>,user_id: i32,username: &str,email: &str,token_auth: &str) -> Result<(), Box<dyn std::error::Error>> {
    let end_point = "CreateUser";
    let payload = serde_json::json!([user_id, username, email, token_auth]);
    let json = serde_json::to_string(&serde_json::json!({ end_point: payload }))?;

    let mut inner = RequestUserCommunicate {
        serialized_request: json,
    };

    let request = Request::new(inner);

    let response = client.user_communicate(request).await?;
    let reply = response.into_inner();

    if reply.success {
        println!("create user success");
        Ok(())
    } else {
        Err(format!("Server error: {}", reply.error).into())
    }
}


async fn call_endpoint(
    client: &mut ChiralClient<Channel>,
    end_point: &str,
    payload: serde_json::Value,
    email: &str,
    token_auth: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let content = serde_json::to_string(&serde_json::json!({ end_point: payload }))?;

    let mut request = Request::new(RequestUserCommunicate {
        serialized_request: content,
    });

    request.metadata_mut().insert("user_id", MetadataValue::from_str(email)?);
    request.metadata_mut().insert("auth_token", MetadataValue::from_str(token_auth)?);

    let reply = client.user_communicate(request).await?.into_inner();

    if reply.success {
        let data: serde_json::Value = serde_json::from_str(&reply.serialized_reply)?;
        Ok(data[end_point].clone())
    } else {
        Err(format!("Server error: {}", reply.error).into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = create_client("http://[::1]:50051").await?;

    say_hello(&mut client, "Rust Client").await?;

    let email = "user@example.com";
    let token = "auth_token_value";

    let result = call_endpoint(
        &mut client,
        "SubmitTestJob",
        serde_json::json!(["gromacs_bench_mem", 0]),
        email,
        token
    ).await?;

    println!("SubmitTestJob response: {}", result);

    Ok(())
}
