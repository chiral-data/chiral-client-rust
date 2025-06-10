use crate::api::create_client;

#[cfg(test)]
mod tests{
    use super::create_client;
    #[tokio::test]
    async fn test_create_client(){
        dotenvy::from_filename(".env").ok();

        let url = std::env::var("CHIRAL_STAGING_API_URL").expect("CHIRAL_STAGING_API_URL environment variable not set");
        let result = create_client(&url).await;
        assert!(result.is_ok(), "Client creation failed: {:?}", result.err());
    }
}