use tonic::transport::Channel;
pub use crate::proto::chiral; 
pub use chiral::chiral_client::ChiralClient;


pub async fn create_client(url: &str) -> Result<ChiralClient<Channel>, Box<dyn std::error::Error>> {
    Ok(ChiralClient::connect(url.to_string()).await?)
}

#[cfg(test)]
mod tests {
    use super::create_client;

    #[tokio::test]
    async fn test_create_client() {
        // Load `.env` file only if CHIRAL_STAGING_API_URL is not already set
        if std::env::var("CHIRAL_STAGING_API_URL").is_err() {
            dotenvy::from_filename(".env").ok();
        }

        let url = std::env::var("CHIRAL_STAGING_API_URL")
            .expect("CHIRAL_STAGING_API_URL environment variable not set or is empty");
        
        assert!(
            !url.trim().is_empty(),
            "CHIRAL_STAGING_API_URL is set but empty"
        );

        let result = create_client(&url).await;
        assert!(result.is_ok(), "Client creation failed: {:?}", result.err());
    }
}

