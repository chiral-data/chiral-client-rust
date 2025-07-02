use ftp::FtpStream;
use std::io::Write;
use std::fs::File;

// #[derive(Debug, PartialEq)]
// enum PathType {
//     NotExist,
//     File,
//     Directory,
// }

pub struct FtpClient {
    ftp_addr: String,
    ftp_port: u16,
    user_email: String,
    token_api: String,
    user_id: String,
    ftp: Option<FtpStream>,
    root_dir: Option<String>,
}

impl FtpClient {
    pub fn new(addr: &str, port: u16, email: &str, token: &str, user_id: &str) -> Self {
        FtpClient {
            ftp_addr: addr.to_string(),
            ftp_port: port,
            user_email: email.to_string(),
            token_api: token.to_string(),
            user_id: user_id.to_string(),
            ftp: None,
            root_dir: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), ftp::FtpError> {
        let address = format!("{}:{}", self.ftp_addr, self.ftp_port);
        let mut ftp_stream = FtpStream::connect(address)?;
        ftp_stream.login(&self.user_email, &self.token_api)?;
        
        // Try to change into the user's subdirectory
        ftp_stream.cwd(&self.user_id)?;
        
        // Confirm we're in the correct directory
        let current_dir = ftp_stream.pwd()?;
        println!("Connected and changed to directory: {current_dir}");

        self.root_dir = Some(current_dir);
        self.ftp = Some(ftp_stream);
        Ok(())
    }


    pub fn disconnect(&mut self) {
        if let Some(mut ftp) = self.ftp.take() {
            let _ = ftp.quit(); // ignore error
        }
        self.root_dir = None;
    }

    pub fn is_connected(&self) -> bool {
    self.ftp.is_some()
    }

    pub fn download_file(&mut self, remote_path: &str, local_path: &str) -> Result<(), ftp::FtpError> {
        let ftp_stream = match &mut self.ftp {
            Some(ftp) => ftp,
            None => {
                return Err(ftp::FtpError::ConnectionError(
                    std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to FTP server"),
                ))
            }
        };

        let data = ftp_stream.simple_retr(remote_path)?;

        let mut file = File::create(local_path).map_err(|e| {
            ftp::FtpError::ConnectionError(e)
        })?;

        file.write_all(&data.into_inner()).map_err(|e| {
            ftp::FtpError::ConnectionError(e)
        })?;

        Ok(())
    }

    pub fn upload_file(&mut self, local_path: &str, remote_path: &str) -> Result<(), ftp::FtpError> {
        let ftp_stream = match &mut self.ftp {
            Some(ftp) => ftp,
            None => {
                return Err(ftp::FtpError::ConnectionError(
                    std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to FTP server"),
                ))
            }
        };

        let mut file = File::open(local_path).map_err(ftp::FtpError::ConnectionError)?;
        ftp_stream.put(remote_path, &mut file)?;

        Ok(())
    }


    pub fn make_directory(&mut self, dir_name: &str) -> Result<(), ftp::FtpError> {
        let ftp_stream = match &mut self.ftp {
            Some(ftp) => ftp,
            None => {
                return Err(ftp::FtpError::ConnectionError(
                    std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to FTP server"),
                ))
            }
        };

        ftp_stream.mkdir(dir_name)?;
        println!("Created directory: {dir_name}");

        Ok(())
    }


    pub fn change_directory(&mut self, dir: &str) -> Result<(), ftp::FtpError> {
        let ftp_stream = match &mut self.ftp {
            Some(ftp) => ftp,
            None => {
                return Err(ftp::FtpError::ConnectionError(
                    std::io::Error::new(std::io::ErrorKind::NotConnected, "Not connected to FTP server"),
                ))
            }
        };  
        ftp_stream.cwd(dir)?;
        let current_dir = ftp_stream.pwd()?;
        self.root_dir = Some(current_dir.clone());

        println!("Changed directory to: {current_dir}");

        Ok(())
    }

    pub fn remove_directory_recursive(&mut self, dir_path: &str) -> Result<(), ftp::FtpError> {
        let ftp_stream = self.ftp.as_mut().ok_or_else(|| {
            ftp::FtpError::ConnectionError(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Not connected to FTP server",
            ))
        })?;

        Self::delete_recursive(ftp_stream, dir_path)
    }

    fn delete_recursive(ftp: &mut FtpStream, dir_path: &str) -> Result<(), ftp::FtpError> {
        println!("Processing directory: {dir_path}");
        
        let entries = match ftp.nlst(Some(dir_path)) {
            Ok(entries) => entries,
            Err(e) => {
                println!("Could not list directory {dir_path}: {e:?}");
                return Err(e);
            }
        };
        
        println!("Found {} entries in {}", entries.len(), dir_path);
        
        for entry in entries {
            if entry.ends_with("/.") || entry.ends_with("/..") {
                continue;
            }
            
            let entry_name = if entry.starts_with(dir_path) {
                entry.strip_prefix(dir_path)
                    .unwrap_or(&entry)
                    .trim_start_matches('/')
            } else if entry.contains('/') {
                entry.split('/').last().unwrap_or(&entry)
            } else {
                &entry
            };
            
            if entry_name.is_empty() || entry_name == "." || entry_name == ".." {
                continue;
            }
            
            let full_path = format!("{}/{}", dir_path.trim_end_matches('/'), entry_name);
            
            match ftp.size(&full_path) {
                Ok(size) => {
                    println!("Deleting file: {full_path} (size: {size:?} bytes)");
                    match ftp.rm(&full_path) {
                        Ok(_) => println!("Deleted file: {full_path}"),
                        Err(e) => {
                            println!("Failed to delete file {full_path}: {e:?}");
                            return Err(e);
                        }
                    }
                },
                Err(_) => {
                    match ftp.nlst(Some(&full_path)) {
                        Ok(_) => {
                            println!("Found subdirectory: {full_path}");
                            Self::delete_recursive(ftp, &full_path)?;
                        },
                        Err(_) => {
                            println!("Attempting to delete as file: {full_path}");
                            match ftp.rm(&full_path) {
                                Ok(_) => println!("Deleted file: {full_path}"),
                                Err(e) => {
                                    println!("Could not delete {full_path}: {e:?}");
                                    return Err(e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        println!("Removing directory: {dir_path}");
        match ftp.rmdir(dir_path) {
            Ok(_) => {
                println!("Successfully deleted directory: {dir_path}" );
                Ok(())
            },
            Err(e) => {
                println!("Failed to delete directory {dir_path}: {e:?}" );
                Err(e)
            }
        }
    }


    pub fn remove_file(&mut self, path: &str) -> Result<(), ftp::FtpError> {
    let ftp_stream = self.ftp.as_mut().ok_or_else(|| {
        ftp::FtpError::ConnectionError(std::io::Error::new(
            std::io::ErrorKind::NotConnected,
            "Not connected to FTP server",
        ))
    })?;

    ftp_stream.rm(path)
}


}

impl Drop for FtpClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use dotenvy::dotenv;
    use std::env;
    use uuid::Uuid;
    use super::*;

    #[test]
    fn test_connection() {
        dotenv().ok(); 

        let ftp_addr = env::var("FTP_HOST").expect("FTP_HOST not set");
        let ftp_port: u16 = env::var("FTP_PORT")
            .expect("FTP_PORT not set")
            .parse()
            .expect("FTP_PORT must be a valid number");

        let ftp_user = env::var("FTP_USER").expect("FTP_USER not set");
        let ftp_pass = env::var("FTP_PASS").expect("FTP_PASS not set");
        let ftp_user_dir = env::var("FTP_USER_DIR").expect("FTP_USER_DIR not set");

        let mut client = FtpClient::new(&ftp_addr, ftp_port, &ftp_user, &ftp_pass, &ftp_user_dir);
        client.connect().expect("Failed to connect to FTP server");
        assert!(client.is_connected());
        client.disconnect();
        assert!(!client.is_connected());
    }

    #[test]
    fn test_file_upload_and_download() {
        dotenv().ok();

        let ftp_addr = env::var("FTP_HOST").expect("FTP_HOST not set");
        let ftp_port: u16 = env::var("FTP_PORT")
            .expect("FTP_PORT not set")
            .parse()
            .expect("FTP_PORT must be a valid number");

        let ftp_user = env::var("FTP_USER").expect("FTP_USER not set");
        let ftp_pass = env::var("FTP_PASS").expect("FTP_PASS not set");
        let ftp_user_dir = env::var("FTP_USER_DIR").expect("FTP_USER_DIR not set");

        let mut client = FtpClient::new(&ftp_addr, ftp_port, &ftp_user, &ftp_pass, &ftp_user_dir);
        client.connect().expect("Failed to connect to FTP server");
        // Ensure 'upload' directory exists
        client.make_directory("upload").ok();
        // Prepare test file
        let local_path = "test_upload.txt";
        let mut file = File::create(local_path).unwrap();
        writeln!(file, "Hello FTP test").unwrap();
        let remote_path = "upload/test_upload.txt";

        // Upload
        client.upload_file(local_path, remote_path).expect("Upload failed");
        // Download
        let download_path = "downloaded_test_upload.txt";
        client.download_file(remote_path, download_path).expect("Download failed");
        // Verify
        let content = fs::read_to_string(download_path).unwrap();
        assert!(content.contains("Hello FTP test"));
        // Cleanup
        fs::remove_file(local_path).unwrap();
        fs::remove_file(download_path).unwrap();
        let _ = client.remove_directory_recursive(remote_path);
        client.disconnect();
    }

    #[test]
    fn test_make_and_change_directory() {
        dotenv().ok();

        let ftp_addr = env::var("FTP_HOST").expect("FTP_HOST not set");
        let ftp_port: u16 = env::var("FTP_PORT")
            .expect("FTP_PORT not set")
            .parse()
            .expect("FTP_PORT must be a valid number");

        let ftp_user = env::var("FTP_USER").expect("FTP_USER not set");
        let ftp_pass = env::var("FTP_PASS").expect("FTP_PASS not set");
        let ftp_user_dir = env::var("FTP_USER_DIR").expect("FTP_USER_DIR not set");

        let mut client = FtpClient::new(&ftp_addr, ftp_port, &ftp_user, &ftp_pass, &ftp_user_dir);
        client.connect().expect("Failed to connect to FTP server");

        let parent = "upload";
        if let Err(e) = client.make_directory(parent) {
            println!("Warning: could not create parent dir '{}': {}", parent, e);
        }

        let uuid = Uuid::new_v4();
        let dir = format!("upload/test_dir_{}", uuid);
        client.make_directory(&dir).expect("Failed to create unique test_dir");

        println!("Directory Made");

        client.change_directory(&dir).expect("Failed to change directory");
        println!("Directory Changed");

        assert!(client.is_connected());
        client.disconnect();
    }

    #[test]
    fn test_recursive_delete_directory() {
        dotenv().ok();
        println!("Loaded .env configuration");

        let ftp_addr = env::var("FTP_HOST").expect("FTP_HOST not set");
        let ftp_port: u16 = env::var("FTP_PORT")
            .expect("FTP_PORT not set")
            .parse()
            .expect("FTP_PORT must be a valid number");
        let ftp_user = env::var("FTP_USER").expect("FTP_USER not set");
        let ftp_pass = env::var("FTP_PASS").expect("FTP_PASS not set");
        let ftp_user_dir = env::var("FTP_USER_DIR").expect("FTP_USER_DIR not set");

        let mut client = FtpClient::new(&ftp_addr, ftp_port, &ftp_user, &ftp_pass, &ftp_user_dir);
        client.connect().expect("Failed to connect to FTP server");

        // Generate unique test directories
        let uuid = Uuid::new_v4();
        let root_dir = format!("upload/test_del_{}", uuid);
        let sub_dir = format!("{}/nested", root_dir);

        // Create directory structure on FTP
        client.make_directory("upload").ok(); // ignore error if exists
        client.make_directory(&root_dir).expect("Could not create root dir");
        client.make_directory(&sub_dir).expect("Could not create nested dir");

        // Create and upload two files (one in each dir)
        let file1 = "temp_root.txt";
        let file2 = "temp_nested.txt";
        fs::write(file1, "Root level file").unwrap();
        fs::write(file2, "Nested file").unwrap();

        let remote1: String = format!("{}/file1.txt", root_dir);
        let remote2 = format!("{}/file2.txt", sub_dir);
        client.upload_file(file1, &remote1).expect("Upload file1 failed");
        client.upload_file(file2, &remote2).expect("Upload file2 failed");

        println!("Files uploaded to test directories");

        // Delete everything
        client.remove_directory_recursive(&root_dir).expect("Recursive deletion failed");
        println!("Recursive deletion complete for {}", root_dir);

        // Local cleanup
        fs::remove_file(file1).ok();
        fs::remove_file(file2).ok();

        client.disconnect();
        println!("Disconnected");
    }
}
