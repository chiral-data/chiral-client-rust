use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue, metadata::MetadataMap};
use std::str::FromStr;
use serde_json::json;

pub mod chiral {
    tonic::include_proto!("chiral"); 
}

use chiral::chiral_client::ChiralClient;
use chiral::{HelloRequest, RequestUserCommunicate};

async fn create_client(url: &str) -> Result<ChiralClient<Channel>, Box<dyn std::error::Error>> {
    println!("creating client..");
    Ok(ChiralClient::connect(url.to_string()).await?)
}
/*
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

fn auth_meta(email: &str, token_auth: &str) -> Result<MetadataMap, Box<dyn std::error::Error>> {
    let mut metadata = MetadataMap::new();
    metadata.insert("user_id", MetadataValue::from_str(email)?);
    metadata.insert("auth_token", MetadataValue::from_str(token_auth)?);
    Ok(metadata)
}

pub fn attach_auth_meta(payload: RequestUserCommunicate, email: &str, token_auth: &str) -> Result<Request<RequestUserCommunicate>, Box<dyn std::error::Error>> {
    let mut req = Request::new(payload);
    req.metadata_mut().insert("user_id", MetadataValue::from_str(email)?);
    req.metadata_mut().insert("auth_token", MetadataValue::from_str(token_auth)?);
    Ok(req)
}

pub async fn confirm_payment(client: &mut ChiralClient<Channel>, email : &str, token_auth: &str, order_id: &str, access_id: &str, amount :i32)->Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "ConfirmPayment";
    let payload = json!([order_id, access_id, amount]);
    let response = call_endpoint(client, end_point, payload, email, token_auth).await?;
    Ok(response)
}

*/
async fn call_endpoint(client: &mut ChiralClient<Channel>,end_point: &str,payload: serde_json::Value,email: &str,token_auth: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
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

pub async fn submit_test_job(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, job_type_name: &str, index: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let end_point = "SubmitTestJob";

    // Create payload
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

    // Await gRPC call
    let response = client.user_communicate(request).await?.into_inner();

    // Check if serialized_reply is non-empty and parse it
    if !response.serialized_reply.trim().is_empty() {
        let parsed: serde_json::Value = serde_json::from_str(&response.serialized_reply)?;

        if let Some(result) = parsed.get(end_point) {
            return Ok(result.clone());
        } else {
            return Err("Missing endpoint data in server response".into());
        }
    }

    // If the serialized reply is empty, check for an error
    if !response.error.trim().is_empty() {
        return Err(format!("Server error: {}", response.error).into());
    }

    Err("Unexpected empty response from server".into())
}

pub async fn get_jobs(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, offset: u32, count_per_page: u32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {    let end_point = "GetJobs";
    println!("hello");
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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename(".env").ok();
    let url = std::env::var("CHIRAL_STAGING_API_URL")?;
    let user_id = std::env::var("TEST_ID")?;
    let username = std::env::var("TEST_USERNAME")?;
    let email = std::env::var("TEST_EMAIL")?;
    let token_auth = std::env::var("TEST_TOKEN_AUTH")?;
    let token_api = std::env::var("TEST_TOKEN_API")?;
    let mut client = create_client(&url).await?;
    println!("client created");

    let order_id = std::env::var("TEST_ORDER_ID")?;
    let access_id = std::env::var("TEST_ACCESS_ID")?;
    let amount: i32 = std::env::var("TEST_PAYMENT_AMOUNT")?.parse()?;


    /*println!("{}", url);
    println!("{}", username);
    println!("{}", user_id);
    println!("{}", email);
    println!("{}", token_auth);
    println!("{}", token_api);
    */

    /*
    SUBMIT TEST JOB TESTING
    let job_type_name = "gromacs_bench_mem"; // Or any other job type you want to test
    let index = 0;
    
    let result = submit_test_job(&mut client, &email, &token_auth, job_type_name, index).await;
    
    // Print result
    match result {
        Ok(response_json) => println!("Server response: {}", response_json),
        Err(e) => eprintln!(" Error: {}", e),
    }
    */
    
    /*
   
    GET JOBS TESTING
    let result = get_jobs(&mut client, &email, &token_auth, 0, 10).await;
    
    match result {
        Ok(response_json) => println!("GetJobs response:\n{}", response_json),
        Err(e) => eprintln!("Error calling GetJobs: {}", e),
    }
    */
    
    /*
    GET JOB TESTING
    let job_id = "jsefa3v6fs7smgikxwlplisrgvxacod7"; // Replace with a real job ID
    let result = get_job(&mut client, &email, &token_auth, job_id).await;
    
    match result {
        Ok(response_json) => println!("GetJob response:\n{}", response_json),
        Err(e) => eprintln!("Error calling GetJob: {}", e),
    }
    */
    

    Ok(())
}
