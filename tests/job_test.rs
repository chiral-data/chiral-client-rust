use chiral_client::{create_client, get_job, submit_job};
mod common;

#[tokio::test]
async fn job_integration_test(){
    let url = common::get_url();
    let mut client = create_client(&url).await.expect("Failed to create client");
    let email = common::get_user_email();
    let token_auth = common::get_token_auth();
    let project_name: &str = "qasdfkjabdi";
    let input_files = vec!["run.sh", "1aki.pdb"];
    let output_files = vec!["1AKI_processed.gro", "topol.top", "posre.itp"];

    let job_id = submit_job(&mut client, &email, &token_auth, "sh run.sh", project_name, &input_files, &output_files).await.expect("submit_job failed");

    let job_id_str = job_id.as_str().expect("Expected job ID to be a string");
    let job_name_retrieved = get_job(&mut client, &email, &token_auth, job_id_str ).await.expect("Failed to get job");

    assert_eq!(job_name_retrieved.get("project_name").and_then(|v| v.as_str()).unwrap_or("unknown"), project_name );
}
