use chiral_client::{
    create_client,
    list_of_projects,
    list_of_example_projects,
    import_example_project,
    list_of_project_files,
    get_project_files,
};
mod common;

#[tokio::test]
async fn full_project_flow_integration_test() {
    let url = common::get_url();
    let email = common::get_user_email();
    let token = common::get_token_auth();

    let mut client = create_client(&url).await.expect("Failed to create client");

    let user_projects = list_of_projects(&mut client, &email, &token)
        .await
        .expect("Failed to list user projects");

    println!("User projects: {}", user_projects);

    let example_projects = list_of_example_projects(&mut client, &email, &token)
        .await
        .expect("Failed to list example projects");

    let first_example = example_projects
        .as_array()
        .expect("Expected example projects to be an array")
        .first()
        .and_then(|v| v.as_str())
        .expect("No example project available");

    println!("First example project: {}", first_example);

    let already_imported = user_projects
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .any(|p| p.as_str() == Some(first_example));

    if !already_imported {
        let import_result = import_example_project(&mut client, &email, &token, first_example)
            .await
            .expect("Failed to import example project");
        println!("Import result: {}", import_result);
    } else {
        println!("Project '{}' already imported. Skipping import.", first_example);
    }

    let file_list = list_of_project_files(&mut client, &email, &token, first_example)
        .await
        .expect("Failed to list files in example project");

    println!("File list: {}", file_list);

    let first_file = file_list
        .as_array()
        .expect("Expected file list to be an array")
        .first()
        .and_then(|v| v.as_str())
        .expect("No files found in the example project");

    println!("First file in project: {}", first_file);

    let file_contents = get_project_files(&mut client, &email, &token, first_example, first_file)
        .await
        .expect("Failed to fetch file from project");

    println!("File contents retrieved (truncated): {:?}", &file_contents);

    assert!(
        file_contents.is_string() || file_contents.is_object(),
        "Expected file contents to be string or JSON object"
    );
}
