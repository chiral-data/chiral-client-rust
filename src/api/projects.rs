use tonic::transport::Channel;
use tonic::{Request, metadata::MetadataValue};
use std::str::FromStr;
use crate::api::client::chiral::chiral_client::ChiralClient;
use crate::api::client::chiral::RequestUserCommunicate;

pub async fn list_of_projects(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "ListOfProjects";
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

pub async fn list_of_example_projects(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "ListOfExampleProjects";
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

pub async fn list_of_project_files(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, project_name: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "ListOfProjectFiles";
    let serialized = format!("{{\"{end_point}\": \"{project_name}\"}}");
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

pub async fn import_example_project(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, project_name: &str)->  Result<serde_json::Value, Box<dyn std::error::Error>>{
    let end_point = "ImportExampleProject";
    let serialized = format!("{{\"{end_point}\": \"{project_name}\"}}");
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

pub async fn get_project_files(client: &mut ChiralClient<Channel>, email: &str, token_auth: &str, project_name: &str, file_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {    let _end_point = "GetJobs";
    let end_point = "GetProjectFile";
    let serialized = format!("{{\"{end_point}\": [\"{project_name}\", \"{file_name}\"]}}");
    let req_msg = RequestUserCommunicate {
        serialized_request: serialized.clone(),
    };

    println!("Sending payload: {serialized}" ); 
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

#[cfg(test)]
mod tests{
    use super::*;
    use crate::api::create_client;
    use dotenvy;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[tokio::test]
    async fn test_list_of_projects() {
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("Missing env").trim() .to_string();
        let email = std::env::var("TEST_EMAIL").expect("Missing env").trim() .to_string();
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("Missing env").trim() .to_string();
        let mut client = create_client(&url).await.expect("Failed to create API client.");

        let projects = list_of_projects(&mut client, &email, &token_auth).await.expect("List of projects failed");

        assert!(projects.is_array(),"Expected JSON array but got: {projects}");

        let projects_array = projects.as_array().expect("Projects response is not a valid JSON array");

        for (i, project) in projects_array.iter().enumerate() {
            match project {
                serde_json::Value::Object(obj) => {
                    assert!(obj.get("id").is_some(),"Project at index {i} missing 'id': {project}");
                    assert!(obj.get("name").is_some(),"Project at index {i} missing 'name': {project}");
                }
                serde_json::Value::String(s) => {
                    assert!(!s.trim().is_empty(),"Project at index {i} is an empty string");
                }
                _ => {
                    panic!("Project at index {i} is neither object nor string: {project}");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_list_of_example_projects(){
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("Missing env").trim() .to_string();
        let email = std::env::var("TEST_EMAIL").expect("Missing env").trim() .to_string();
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("Missing env").trim() .to_string();
        let mut client = create_client(&url).await.expect("Failed to create API client.");

        let projects = list_of_example_projects(&mut client, &email, &token_auth).await.expect("List of projects failed");

        assert!(projects.is_array(),"Expected JSON array but got: {projects}");
        let projects_array = projects.as_array().expect("Projects response is not a valid JSON array");
        for (i, project) in projects_array.iter().enumerate() {
            match project {
                serde_json::Value::Object(obj) => {
                    assert!(obj.get("id").is_some(),"Project at index {i} missing 'id': {project}");
                    assert!(obj.get("name").is_some(),"Project at index {i} missing 'name': {project}");
                }
                serde_json::Value::String(s) => {
                    assert!(!s.trim().is_empty(),"Project at index {i} is an empty string");
                }
                _ => {
                    panic!("Project at index {i} is neither object nor string: {project}");
                }
            }
        }

    }

    #[tokio::test]
    async fn test_list_of_project_files(){
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("Missing env").trim() .to_string();
        let email = std::env::var("TEST_EMAIL").expect("Missing env").trim() .to_string();
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("Missing env").trim() .to_string();
        let mut client = create_client(&url).await.expect("Failed to create API client.");
        let example_projects = list_of_example_projects(&mut client, &email, &token_auth).await.expect("Failed to get example projects");
        let project_name_opt = example_projects.as_array().and_then(|arr| {
        arr.iter().find_map(|p| match p {
                serde_json::Value::String(name) => Some(name.clone()),
                serde_json::Value::Object(obj) => obj.get("name").and_then(|v| v.as_str()).map(String::from),
                _ => None,
            })
        });

        let project_name = project_name_opt.expect("No valid project name found in example projects");
        let files = list_of_project_files(&mut client, &email, &token_auth, &project_name).await.expect("Failed to get project files");

        assert!(files.is_array(),"Expected project files to be a JSON array, got: {files}");

        let file_array = files.as_array().unwrap();
        println!("Found {} file(s) in project '{}'", file_array.len(), project_name);
        
        for (i, file) in file_array.iter().enumerate() {
            assert!(file.is_string(),"Expected file entry at index {i} to be a string, got: {file}");
        }
    }

    #[tokio::test]
    async fn test_import_example_project() {
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("Missing env").trim() .to_string();
        let email = std::env::var("TEST_EMAIL").expect("Missing env").trim() .to_string();
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("Missing env").trim() .to_string();
        let mut client = create_client(&url).await.expect("Failed to create API client");

        let existing_projects = list_of_projects(&mut client, &email, &token_auth).await.expect("Failed to fetch list of projects");
        let example_projects = list_of_example_projects(&mut client, &email, &token_auth).await.expect("Failed to fetch list of example projects");

        let example_project_name = example_projects
            .as_array()
            .and_then(|arr| {
                arr.iter().find_map(|p| match p {
                    serde_json::Value::String(name) => Some(name.clone()),
                    serde_json::Value::Object(obj) => obj.get("name").and_then(|v| v.as_str()).map(String::from),
                    _ => None,
                })
            }).expect("No valid example project name found");

        let already_exists = existing_projects
            .as_array()
            .map(|arr| {
                arr.iter().any(|p| match p {
                    serde_json::Value::String(name) => name == &example_project_name,
                    serde_json::Value::Object(obj) => obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s == example_project_name)
                        .unwrap_or(false),
                    _ => false,
                })
            })
            .unwrap_or(false);

        if !already_exists {
            let result = import_example_project(&mut client, &email, &token_auth, &example_project_name).await.expect("Failed to import example project");
            assert!(result.is_object() || result.is_string(),"Expected object or string in response, got: {result}");
        }
    }
    
    #[tokio::test]
    async fn test_get_project_files() {
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("Missing env").trim() .to_string();
        let email = std::env::var("TEST_EMAIL").expect("Missing env").trim() .to_string();
        let token_auth = std::env::var("TEST_TOKEN_AUTH").expect("Missing env").trim() .to_string();
        let mut client = create_client(&url).await.expect("Failed to create API client");

        let projects = list_of_projects(&mut client, &email, &token_auth)
            .await
            .expect("Failed to fetch list of projects");

        let project_name = projects
            .as_array()
            .and_then(|arr| {
                let mut rng = thread_rng();
                let valid_names: Vec<String> = arr
                    .iter()
                    .filter_map(|p| match p {
                        serde_json::Value::Object(obj) => obj.get("name").and_then(|v| v.as_str()).map(String::from),
                        serde_json::Value::String(name) => Some(name.clone()),
                        _ => None,
                    })
                    .filter(|name| !name.starts_with("gromacs_bench"))
                    .collect();

                valid_names.choose(&mut rng).cloned()
            })
            .expect("No valid project name found (after filtering broken gromacs_bench projects)");

        println!("Using project: {project_name}");
        let files = list_of_project_files(&mut client, &email, &token_auth, &project_name)
            .await
            .expect("Failed to list project files");

        let file_array = files.as_array().expect("File list is not an array");

        println!("Found {} file(s) in project.", file_array.len());
        let mut at_least_one_success = false;

        for file in file_array {
            let file_name = match file {
                serde_json::Value::Object(obj) => obj.get("name").and_then(|v| v.as_str()).map(String::from),
                serde_json::Value::String(name) => Some(name.clone()),
                _ => None,
            };

            let Some(file_name) = file_name else {
                println!("Skipping invalid file entry: {file:?}");
                continue;
            };

            match get_project_files(&mut client, &email, &token_auth, &project_name, &file_name).await {
                Ok(file_data) => {
                    assert!(
                        file_data.is_string() || file_data.is_object(),
                        "Expected JSON string or object for file '{file_name}', got: {file_data}",
                    );
                    at_least_one_success = true;
                }
                Err(e) => {
                    println!("Error fetching '{file_name}': {e}");
                }
            }
        }

        assert!(
            at_least_one_success,
            "No project file could be successfully fetched and validated."
        );

    }
}
