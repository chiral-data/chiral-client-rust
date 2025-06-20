use dotenvy;
use std::env;

pub fn load_env() {
    dotenvy::from_filename(".env").expect("Failed to load .env");
}

pub fn get_api_url() -> String {
    load_env();
    env::var("CHIRAL_STAGING_API_URL").expect("CHIRAL_STAGING_API_URL not set")
}
