use tonic::transport::Channel;
pub use crate::proto::chiral; 
pub use chiral::chiral_client::ChiralClient;

pub async fn create_client(url: &str) -> Result<ChiralClient<Channel>, Box<dyn std::error::Error>> {
    Ok(ChiralClient::connect(url.to_string()).await?)
}

#[cfg(test)]
mod tests{
    use super::create_client;
    #[tokio::test]
    async fn test_create_client(){
        dotenvy::from_filename(".env.staging").ok();
        let url = std::env::var("CHIRAL_STAGING_API_URL")
        .expect("Missing env")
        .trim() 
        .to_string();

        println!("CHIRAL_STAGING_API_URL = {:?}", url);
        let result = create_client(&url).await;
        assert!(result.is_ok(), "Client creation failed: {:?}", result.err());
    }
}
