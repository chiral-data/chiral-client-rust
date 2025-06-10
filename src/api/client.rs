use tonic::transport::Channel;

pub mod chiral {
    tonic::include_proto!("chiral"); 
}

use chiral::chiral_client::ChiralClient;

pub async fn create_client(url: &str) -> Result<ChiralClient<Channel>, Box<dyn std::error::Error>> {
    Ok(ChiralClient::connect(url.to_string()).await?)
}