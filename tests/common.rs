use std::env;

pub fn load_env() {
    dotenvy::from_filename(".env").expect("Failed to load .env");
}

#[allow(dead_code)]
pub fn get_url() -> String {
    load_env();
    env::var("CHIRAL_STAGING_API_URL").expect("CHIRAL_STAGING_API_URL not set")
}

#[allow(dead_code)]
pub fn get_user_email() -> String {
    load_env();
    env::var("TEST_EMAIL").expect("TEST_EMAIL not set")
}

#[allow(dead_code)]
pub fn get_token_auth() -> String {
    load_env();
    env::var("TEST_TOKEN_AUTH").expect("TEST_TOKEN_AUTH not set")
}
