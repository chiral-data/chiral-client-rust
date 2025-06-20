// TODO: write integration tests
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html
//

pub mod proto; 
pub use crate::proto::chiral;
pub mod api;

pub use api::{
    create_client,
    get_credit_points,
    submit_job,
    get_job,
    list_of_example_projects,
    list_of_projects,
    list_of_project_files,
    import_example_project,
    get_project_files,
};