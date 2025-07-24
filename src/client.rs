use crate::{
    create_client,
    get_credit_points,
    submit_job,
    get_job,
    list_of_example_projects,
    list_of_projects,
    list_of_project_files,
    import_example_project,
    get_project_files,
    get_token_api,
    refresh_token_api,
};
use crate::file::FtpClient;
use derivative::Derivative;
use std::env;
use dotenvy::dotenv;
use std::error::Error;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Client {
    url: String,
    user_email: String,
    user_id: String,
    token_auth: String,
    token_api: String,
    api_port: u16,
    ftp_addr: String,
    file_port: u16,
    #[allow(dead_code)]
    #[derivative(Debug = "ignore")]
    pub ftp_session: Option<FtpClient>,
}

impl Client {
    pub fn new(url: String,user_email: String,user_id: String,token_auth: String,token_api: String,api_port:u16 ,ftp_addr: String,file_port: u16) -> Self {
        Self {
            url,
            user_email,
            user_id,
            token_auth,
            token_api,
            api_port,
            ftp_addr,
            file_port,
            ftp_session: None,
        }
    }
    pub async fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok(); // fallback to .env if .env file exists

        let url = env::var("URL")?;
        let user_email = env::var("USER_EMAIL")?;
        let user_id = env::var("USER_ID")?;
        let token_auth = env::var("TOKEN_AUTH")?;
        let ftp_addr = env::var("FTP_ADDR")?;
        let file_port = env::var("FTP_PORT")?.parse::<u16>()?;
        let api_port = env::var("API_PORT")?.parse::<u16>()?;

        let mut client = create_client(&url).await?;
        let response = get_token_api(&mut client, &user_email, &token_auth).await?;
        let token_api = response
            .as_str()
            .ok_or_else(|| "Expected a string in token API response")?
            .to_string();

        Ok(Self::new(
            url,
            user_email,
            user_id,
            token_auth,
            token_api,
            api_port,
            ftp_addr,
            file_port,
        ))
    }

    
    pub async fn get_credit_points(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        get_credit_points(&mut client, &self.user_email, &self.token_auth).await
    }
    pub async fn submit_job(&mut self,command_string: &str,project_name: &str,input_files: &[&str],output_files: &[&str],) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        let result = submit_job(
            &mut client,
            &self.user_email,
            &self.token_auth,
            command_string,
            project_name,
            input_files,
            output_files,
        ).await?;

        Ok(result)
    }
    pub async fn get_job(&mut self, job_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        get_job(&mut client, &self.user_email, &self.token_auth, job_id).await
    }

    pub async fn list_projects(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        list_of_projects(&mut client, &self.user_email, &self.token_auth).await
    }

    pub async fn list_example_projects(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        list_of_example_projects(&mut client, &self.user_email, &self.token_auth).await
    }

    pub async fn get_project_files(&mut self, project_name: &str, file_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        get_project_files(&mut client, &self.user_email, &self.token_auth, project_name, file_name).await
    }

    pub async fn list_project_files(&mut self, project_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        list_of_project_files(&mut client, &self.user_email, &self.token_auth, project_id).await
    }
    pub async fn import_example_project(&mut self, project_name: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        import_example_project(&mut client, &self.user_email, &self.token_auth, project_name).await
    }
    pub async fn get_api_token(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        get_token_api(&mut client, &self.user_email, &self.token_auth).await
    }
    pub async fn refresh_api_token(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut client = create_client(&self.url).await?;
        refresh_token_api(&mut client, &self.user_email, &self.token_api).await
    }

    pub async fn connect_ftp(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut ftp_client = FtpClient::new(
            &self.ftp_addr,
            self.file_port,
            &self.user_email,
            &self.token_api,
            &self.user_id,
        );
        ftp_client.connect()?;
        self.ftp_session = Some(ftp_client); // persist session
        Ok(())
    }
    pub async fn disconnect_ftp(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(mut ftp_client) = self.ftp_session.take() {
            ftp_client.disconnect();
        }
        Ok(())
    }

    pub async fn upload_file(&mut self,local_path: &str,remote_path: &str,) -> Result<(), ftp::FtpError> {
        let ftp_client = self
            .ftp_session
            .as_mut()
            .ok_or_else(|| ftp::FtpError::ConnectionError(
                std::io::Error::new(std::io::ErrorKind::NotConnected, "FTP not connected")
            ))?;

        ftp_client.upload_file(local_path, remote_path)
    }

    pub async fn download_file(&mut self, remote_path: &str, local_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ftp_client = self.ftp_session.as_mut().ok_or("FTP not connected")?;
        ftp_client.download_file(remote_path, local_path)?;
        Ok(())
    }

    pub async fn current_directory(&mut self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let ftp_client = self.ftp_session.as_mut().ok_or("FTP not connected")?;
        let current_dir = ftp_client.current_directory()?;
        Ok(current_dir)
    }

    pub async fn check_if_directory_exists(&mut self, dir_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let ftp_client = self.ftp_session.as_mut().ok_or("FTP not connected")?;
        let exists = ftp_client.check_if_directory_exists(dir_path)?;
        Ok(exists)
    }
    pub async fn make_directory(&mut self, dir_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ftp_client = self.ftp_session.as_mut().ok_or("FTP not connected")?;
        ftp_client.make_directory(dir_name)?;
        Ok(())
    }

    pub async fn change_directory(&mut self, dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let ftp_client = self.ftp_session.as_mut().ok_or("FTP not connected")?;
        ftp_client.change_directory(dir_path)?;
        Ok(())
    }

}


