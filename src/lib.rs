// TODO: write integration tests
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html
//

pub mod proto; 
pub use crate::proto::chiral;
pub mod api;

pub use api::create_client;
pub use api::get_credit_points;
pub use api::{
    submit_job,
    get_job,
};