use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue, metadata::MetadataMap};
use std::str::FromStr;
use serde_json::json;

pub mod chiral {
    tonic::include_proto!("chiral"); 
}

mod api;

use api::{
    create_client,
    get_credit_points,
    get_token_api,
    refresh_token_api,
    submit_test_job,
    get_jobs,
    get_job,
    list_of_projects,
    list_of_project_files,
    import_example_project,
    list_of_example_projects,
    get_project_files,
    submit_job,
};

use chiral::chiral_client::ChiralClient;
use chiral::{HelloRequest, RequestUserCommunicate};


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
    let project_name: &str = "utils_sleep";
    let file_name: &str = "sleep_30s.sh";

    let command_string = "bash run.sh";
    let project_name = "utils_sleep";
    let input_files = vec!["run.sh"];
    let output_files = vec!["output.txt", "checkpoints.cp"];

    

    Ok(())
}
