use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue, metadata::MetadataMap};
use std::str::FromStr;
use serde_json::json;
use rand::prelude::SliceRandom;
use rand::Rng;

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
    
    let order_id = std::env::var("TEST_ORDER_ID")?;
    let access_id = std::env::var("TEST_ACCESS_ID")?;
    let amount: i32 = std::env::var("TEST_PAYMENT_AMOUNT")?.parse()?;
    let project_name: &str = "qCEnc6q";
    let file_name: &str = "sleep_30s.sh";
    let index: u32 = 1;
    let command_string = "bash run.sh";
    let input_files = vec!["run.sh", "1aki.pdb"];
    
    let output_files = vec!["1AKI_processed.gro", "topol.top", "posre.itp"];
    let job_types = vec![
        "sleep_5_secs", 
        "gromacs_bench_mem", 
    ];
    let job_type_name = job_types.choose(&mut rand::thread_rng()).unwrap();
    let index: u32 = rand::thread_rng().gen_range(0..5);
    let mut client = create_client(&url).await?;
    println!("client created");

    // Get credit points
    let points = get_credit_points(&mut client, &email, &token_auth).await?;
    println!("User {} has {} points", email, points);

    // Get and refresh token API
    let token_api = get_token_api(&mut client, &email, &token_auth).await?;
    println!("Token API: {}", token_api);

    let refreshed_token = refresh_token_api(&mut client, &email, &token_auth).await?;
    println!("Refreshed token: {}", refreshed_token);

    let submit_result = submit_test_job(&mut client, &email, &token_auth, job_type_name, index).await;

    let job_id = match submit_result {
        Ok(response_json) => {
            println!("SubmitTestJob response:\n{}", response_json);
            
            // If the response is just a plain string like `"abc123"`, strip the quotes
            if let Some(stripped) = response_json.as_str() {
                stripped.to_string()
            } else {
                eprintln!("Response is not a plain string.");
                return Ok(());
            }
        },
        Err(e) => {
            eprintln!("Error calling SubmitTestJob: {}", e);
            return Ok(());
        }
    };



    let jobs_result = get_jobs(&mut client, &email, &token_auth, 0, 10).await;

    match jobs_result {
        Ok(response_json) => println!("GetJobs response:\n{}", response_json),
        Err(e) => eprintln!("Error calling GetJobs: {}", e),
    }

    // === GET JOB ===
    let get_job_result = get_job(&mut client, &email, &token_auth, &job_id).await;

    match get_job_result {
        Ok(response_json) => println!("GetJob response:\n{}", response_json),
        Err(e) => eprintln!("Error calling GetJob: {}", e),
    }

    // List projects
    let projects = list_of_projects(&mut client, &email, &token_auth).await?;
    if let Some(projects_array) = projects.as_array() {
        println!("User has {} project(s)", projects_array.len());
    } else {
        println!("Unexpected response format for projects: {}", projects);
    }


    // List and import example projects
    let example_projects = list_of_example_projects(&mut client, &email, &token_auth).await?;
    println!("Available example projects: {:?}", example_projects);
    // For projects
    if let Some(projects_array) = projects.as_array() {
        println!("User has {} project(s)", projects_array.len());
    } else {
        println!("Projects is not an array: {:?}", projects);
    }

    // For example_projects
    if let Some(example_projects_array) = example_projects.as_array() {
        println!("Available example projects: {:?}", example_projects_array);
        if let Some(project_name_val) = example_projects_array.first() {
            if let Some(project_name) = project_name_val.as_str() {
                import_example_project(&mut client, &email, &token_auth, project_name).await?;
                println!("Imported example project: {}", project_name);

                // Remaining logic stays the same
                let project_files = list_of_project_files(&mut client, &email, &token_auth, project_name).await?;
                println!("Project {} has files: {:?}", project_name, project_files);

                let run_sh_content = get_project_files(&mut client, &email, &token_auth, project_name, "run.sh").await?;
                println!("Content of run.sh: {}", run_sh_content);

                let job_id = submit_job(&mut client, &email, &token_auth, "sh run.sh", project_name, &input_files, &output_files).await?;
                println!("Submitted job with ID: {}", job_id);

                let topol_top_content = get_project_files(&mut client, &email, &token_auth, project_name, "topol.top").await?;
                println!("Content of topol.top: {}", topol_top_content);
            }
        }
    }


    Ok(())
}
