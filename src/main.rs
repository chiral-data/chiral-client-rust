// TODO: change this file to lib.rs
//
// TODO: write integration tests
// https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html
//
// TODO: add .gitignore for Rust

use rand::prelude::SliceRandom;
use rand::Rng;
pub mod proto; 
pub use crate::proto::chiral;
mod api;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::from_filename(".env").ok();
    let _url = std::env::var("CHIRAL_STAGING_API_URL")?;
    let _user_id = std::env::var("TEST_ID")?;
    let _username = std::env::var("TEST_USERNAME")?;
    let _email = std::env::var("TEST_EMAIL")?;
    let _token_auth = std::env::var("TEST_TOKEN_AUTH")?;
    let _token_api = std::env::var("TEST_TOKEN_API")?;
    
    let _order_id = std::env::var("TEST_ORDER_ID")?;
    let _access_id = std::env::var("TEST_ACCESS_ID")?;
    let _amount: i32 = std::env::var("TEST_PAYMENT_AMOUNT")?.parse()?;
    let _project_name: &str = "qCEnc6q";
    let _file_name: &str = "sleep_30s.sh";
    let _index: u32 = 1;
    let _command_string = "bash run.sh";
    let _input_files = vec!["run.sh", "1aki.pdb"];
    
    let _output_files = vec!["1AKI_processed.gro", "topol.top", "posre.itp"];
    let job_types = ["sleep_5_secs", 
        "gromacs_bench_mem"];
    let _job_type_name = job_types.choose(&mut rand::thread_rng()).unwrap();
    let _index: u32 = rand::thread_rng().gen_range(0..5);
 

    Ok(())
}
