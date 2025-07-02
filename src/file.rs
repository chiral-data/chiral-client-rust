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

            match ftp.nlst(Some(&full_path)) {
                Ok(_) => {
                    println!("Found subdirectory: {full_path}");
                    Self::delete_recursive(ftp, &full_path)?;
                }
                Err(_) => {
                    println!("Attempting to delete file: {full_path}");
                    match ftp.rm(&full_path) {
                        Ok(_) => println!("Deleted file: {full_path}"),
                        Err(e) => {
                            println!("Could not delete file {full_path}: {e:?}");
                            return Err(e);
                        }
                    }
                }
            }
        }

        println!("Removing directory: {dir_path}");
        match ftp.rmdir(dir_path) {
            Ok(_) => {
                println!("Successfully deleted directory: {dir_path}");
                Ok(())
            }
            Err(e) => {
                println!("Failed to delete directory {dir_path}: {e:?}");
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
    use super::*; // access FtpClient and methods

    use std::fs::{self, File};
    use std::io::Write;
    use std::sync::mpsc;
    use std::thread::{self, JoinHandle};
    use tokio::select;
    use uuid::Uuid;

    use unftp_sbe_fs::Filesystem;
    use libunftp::{ServerBuilder};
    use libunftp::auth::AnonymousAuthenticator;
    
    fn spawn_test_ftp_server_with_shutdown() -> (JoinHandle<()>, String, mpsc::Sender<()>) {
    
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        drop(listener); 
        
        let (shutdown_tx, shutdown_rx) = mpsc::channel();
        
        let handle = thread::spawn({
            let addr_clone = addr.clone();
            move || {
                let backend_factory = || Filesystem::new(std::env::temp_dir());
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async {
                    let server = ServerBuilder::new(Box::new(backend_factory))
                    .authenticator(std::sync::Arc::new(AnonymousAuthenticator {}))
                    .build()
                    .unwrap();
                
                let (async_shutdown_tx, mut async_shutdown_rx) = tokio::sync::mpsc::unbounded_channel();
                let sync_rx = shutdown_rx;
                tokio::spawn(async move {
                    let _ = tokio::task::spawn_blocking(move || sync_rx.recv()).await;
                    let _ = async_shutdown_tx.send(());
                });
                
                select! {
                    result = server.listen(addr_clone) => {
                        if let Err(e) = result {
                            eprintln!("Server error: {}", e);
                        }
                    }
                    _ = async_shutdown_rx.recv() => {
                        println!("Shutdown signal received, stopping server");
                    }
                }
            });
        }
    });
    
    std::thread::sleep(std::time::Duration::from_millis(300));
    (handle, addr, shutdown_tx)
    }

    #[test]
    fn test_connection() {
        
        let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown();
        
        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port in server address");
        
        std::thread::sleep(std::time::Duration::from_millis(300));
        
        let test_credentials = vec![
            ("anonymous", "", "/"),
            ("test", "test", "/"),
            ("ftp", "", "/"),
            ];

            let mut connection_successful = false;
            for (user, pass, dir) in test_credentials {
                let mut client = FtpClient::new(host, port, user, pass, dir);
                
                match client.connect() {
                    Ok(_) => {
                        println!("Connected to FTP server at {} with credentials {}/{}", addr, user, pass);
                        assert!(client.is_connected());
                        
                        client.disconnect();
                        assert!(!client.is_connected());
                        
                        connection_successful = true;
                        break;
                    }
                    Err(e) => {
                        println!("Failed to connect with {}/{}: {}", user, pass, e);
                    }
                }
            }
            
            shutdown_tx.send(()).expect("Failed to send shutdown signal");
            handle.join().expect("Server thread panicked");
            
            assert!(connection_successful, "Failed to connect to the test FTP server with any credentials");
        }
        
        #[test]
        fn test_file_upload_and_download() {
            
            let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown();
            
            let addr_parts: Vec<&str> = addr.split(':').collect();
            let host = addr_parts[0];
            let port: u16 = addr_parts[1].parse().expect("Invalid port in server address");
            
            std::thread::sleep(std::time::Duration::from_millis(300)); 
            
            let user = "anonymous";
            let pass = "";
            let user_dir = "/";
            
            let mut client = FtpClient::new(host, port, user, pass, user_dir);
            client.connect().expect("Failed to connect to test FTP server");
            
            client.make_directory("upload").ok();
            
            let local_path = "test_upload.txt";
            let mut file = File::create(local_path).unwrap();
            writeln!(file, "Hello FTP test").unwrap();
            
            let remote_path = "upload/test_upload.txt";
            client.upload_file(local_path, remote_path).expect("Upload failed");
            
            let download_path = "downloaded_test_upload.txt";
            client.download_file(remote_path, download_path).expect("Download failed");
            
            let content = fs::read_to_string(download_path).unwrap();
            assert!(content.contains("Hello FTP test"));
            
            fs::remove_file(local_path).unwrap();
            fs::remove_file(download_path).unwrap();
            client.remove_directory_recursive("upload").ok();
            client.disconnect();
            
            shutdown_tx.send(()).expect("Failed to send shutdown signal");
            handle.join().expect("Server thread panicked");
        }
        
    #[test]
    fn test_make_and_change_directory() {
        
        let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown();
        
        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port in server address");
        
        std::thread::sleep(std::time::Duration::from_millis(300)); 
        
        let user = "anonymous";
        let pass = "";
        let user_dir = "/";
        
        let mut client = FtpClient::new(host, port, user, pass, user_dir);
        client.connect().expect("Failed to connect to test FTP server");
        
        let parent = "upload";
        if let Err(e) = client.make_directory(parent) {
            println!("Warning: could not create parent dir '{}': {}", parent, e);
        }
        
        let uuid = Uuid::new_v4();
        let dir = format!("upload/test_dir_{}", uuid);
        client.make_directory(&dir).expect("Failed to create unique test_dir");
        
        println!("Directory Made: {dir}");
        
        client.change_directory(&dir).expect("Failed to change directory");
        println!("Directory Changed to: {dir}");
        
        assert!(client.is_connected());
        client.disconnect();
        
        shutdown_tx.send(()).expect("Failed to send shutdown signal");
        handle.join().expect("Server thread panicked");
    }

    #[test]
    fn test_recursive_delete_directory() {
        println!("Loaded .env configuration");
        
        let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown();
        
        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port in server address");
        
        std::thread::sleep(std::time::Duration::from_millis(300)); 
        
        let user = "anonymous";
        let pass = "";
        let user_dir = "/";
        
        let mut client = FtpClient::new(host, port, user, pass, user_dir);
        client.connect().expect("Failed to connect to FTP server");
        
        let uuid = Uuid::new_v4();
        let root_dir = format!("upload/test_del_{}", uuid);
        let sub_dir = format!("{}/nested", root_dir);
        
        client.make_directory("upload").ok(); 
        client.make_directory(&root_dir).expect("Could not create root dir");
        client.make_directory(&sub_dir).expect("Could not create nested dir");
        
        let file1 = "temp_root.txt";
        let file2 = "temp_nested.txt";
        fs::write(file1, "Root level file").unwrap();
        fs::write(file2, "Nested file").unwrap();
        
        let remote1: String = format!("{}/file1.txt", root_dir);
        let remote2 = format!("{}/file2.txt", sub_dir);
        client.upload_file(file1, &remote1).expect("Upload file1 failed");
        client.upload_file(file2, &remote2).expect("Upload file2 failed");
        
        println!("Files uploaded to test directories");
        
        client.remove_directory_recursive(&root_dir).expect("Recursive deletion failed");
        println!("Recursive deletion complete for {}", root_dir);
        
        fs::remove_file(file1).ok();
        fs::remove_file(file2).ok();
        
        client.disconnect();
        println!("Disconnected");
        
        shutdown_tx.send(()).expect("Failed to send shutdown signal");
        handle.join().expect("Server thread panicked");
    }

}