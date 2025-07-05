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
        match ftp_stream.cwd(&self.user_id) {
            Ok(_) => {
                println!("Directory '{}' exists. Switched into it.", self.user_id);
            }
            Err(cwd_error) => {
                println!("Directory '{}' does not exist. Attempting to create...", self.user_id);
                
                // Try to create the directory
                match ftp_stream.mkdir(&self.user_id) {
                    Ok(_) => {
                        println!("Directory '{}' created successfully.", self.user_id);
                        // Now try to change into it
                        match ftp_stream.cwd(&self.user_id) {
                            Ok(_) => {
                                println!("Successfully switched into directory '{}'.", self.user_id);
                            }
                            Err(cwd_error2) => {
                                println!("Warning: Created directory '{}' but couldn't switch into it: {:?}", self.user_id, cwd_error2);
                                println!("Continuing without switching to user directory.");
                            }
                        }
                    }
                    Err(mkdir_error) => {
                        println!("Warning: Could not create directory '{}': {:?}", self.user_id, mkdir_error);
                        println!("Original cwd error: {:?}", cwd_error);
                        println!("Continuing without user directory - working from root.");
                    }
                }
            }
        }

        let current_dir = ftp_stream.pwd()?;
        println!("Connected and current directory: {current_dir}");

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

        // Store current working directory to restore later
        let current_dir = ftp.pwd()?;
        
        let entries = match ftp.nlst(Some(dir_path)) {
            Ok(entries) => entries,
            Err(e) => {
                println!("Could not list directory {dir_path}: {e:?}");
                return Err(e);
            }
        };

        for entry in entries {
            if entry.ends_with("/.") || entry.ends_with("/..") {
                continue;
            }

            let entry_name = if entry.starts_with(dir_path) {
                entry.strip_prefix(dir_path)
                    .unwrap_or(&entry)
                    .trim_start_matches('/')
            } else if entry.contains('/') {
                entry.split('/').next_back().unwrap_or(&entry)
            } else {
                &entry
            };

            if entry_name.is_empty() || entry_name == "." || entry_name == ".." {
                continue;
            }

            let full_path = format!("{}/{}", dir_path.trim_end_matches('/'), entry_name);

            // Distinguish directory vs file by trying cwd
            match ftp.cwd(&full_path) {
                Ok(_) => {
                    // Restore working directory before recursive call
                    ftp.cwd(&current_dir)?;
                    println!("Found subdirectory: {full_path}");
                    Self::delete_recursive(ftp, &full_path)?;
                }
                Err(_) => {
                    println!("Attempting to delete file: {full_path}");
                    ftp.rm(&full_path).map(|_| {
                        println!("Deleted file: {full_path}");
                    }).map_err(|e| {
                        println!("Could not delete file {full_path}: {e:?}");
                        e
                    })?;
                }
            }
        }

        println!("Removing directory: {dir_path}");
        ftp.rmdir(dir_path).map(|_| {
            println!("Successfully deleted directory: {dir_path}");
        }).map_err(|e| {
            println!("Failed to delete directory {dir_path}: {e:?}");
            e
        })
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
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread::{self, JoinHandle};
    use std::time::{Duration, Instant};
    use tokio::select;
    use uuid::Uuid;
    use unftp_sbe_fs::Filesystem;
    use libunftp::{auth::AnonymousAuthenticator, ServerBuilder};

    fn spawn_test_ftp_server_with_shutdown_ready() -> (JoinHandle<()>, String, std::sync::mpsc::Sender<()>) {

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap().to_string();
        drop(listener);

        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
        let (ready_tx, ready_rx) = mpsc::channel::<()>();

        let handle = thread::spawn({
            let addr_clone = addr.clone();
            move || {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async {
                    let backend_factory = || Filesystem::new(std::env::temp_dir());

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

                    let server_task = tokio::spawn({
                        let addr_clone = addr_clone.clone();
                        async move {
                            server.listen(addr_clone).await
                        }
                    });

                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    let _ = ready_tx.send(());

                    select! {
                        result = server_task => {
                            if let Err(e) = result.unwrap() {
                                eprintln!("FTP Server error: {}", e);
                            }
                        }
                        _ = async_shutdown_rx.recv() => {
                            println!("Shutdown signal received, stopping server");
                        }
                    }
                });
            }
        });

        ready_rx.recv().expect("Server failed to start");

        // wait_for_server_ready(&addr);

        (handle, addr, shutdown_tx)
    }

    fn wait_for_server_ready(addr: &str) {
        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port");

        let start_time = Instant::now();
        let timeout = Duration::from_secs(10);

        while start_time.elapsed() < timeout {
            let mut test_client = FtpClient::new(host, port, "anonymous", "", "test_user");
            if test_client.connect().is_ok() {
                test_client.disconnect();
                println!("Server is ready to accept connections at {}", addr);
                return;
            }
            thread::sleep(Duration::from_millis(50));
        }

        panic!("Server did not become ready within timeout period");
    }

    #[test]
    fn test_connection() {
        let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown_ready();

        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port");

        let test_credentials = vec![
            ("anonymous", "", "test_user"),
            ("test", "test", "test_user"),
            ("ftp", "", "test_user"),
        ];  

        let mut connection_successful = false;
        for (user, pass, initial_dir) in test_credentials {
            let mut client = FtpClient::new(host, port, user, pass, initial_dir);

            match client.connect() {
                Ok(_) => {
                    println!(
                        "Connected to FTP server at {} with credentials {}/{} and initial directory {}",
                        addr, user, pass, initial_dir
                    );
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

        shutdown_tx.send(()).expect("Failed to send shutdown");
        handle.join().expect("Server thread panicked");

        assert!(connection_successful, "Failed to connect with any credentials");
    }

    #[test]
    fn test_file_upload_and_download() {
        let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown_ready();

        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port");

        let mut client = FtpClient::new(host, port, "anonymous", "", "test_user");
        client.connect().expect("Failed to connect");

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

        shutdown_tx.send(()).expect("Failed to send shutdown");
        handle.join().expect("Server thread panicked");
    }

    #[test]
fn test_make_and_change_directory() {
    let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown_ready();

    let addr_parts: Vec<&str> = addr.split(':').collect();
    let host = addr_parts[0];
    let port: u16 = addr_parts[1].parse().expect("Invalid port");

    let mut client = FtpClient::new(host, port, "anonymous", "", "test_user");
    client.connect().expect("Failed to connect");

    // Ensure user root is correct
    let user_root = "upload";
    client.make_directory(user_root).ok();

    let uuid = Uuid::new_v4();
    let dir = format!("{}/test_dir_{}", user_root, uuid);
    client.make_directory(&dir).expect("Failed to create dir");

    println!("Directory Made: {dir}");
    client.change_directory(&dir).expect("Failed to change dir");

    assert!(client.is_connected());
    client.disconnect();

    shutdown_tx.send(()).expect("Failed to send shutdown");
    handle.join().expect("Server thread panicked");
}


    #[test]
    fn test_recursive_delete_directory() {
        let (handle, addr, shutdown_tx) = spawn_test_ftp_server_with_shutdown_ready();
        wait_for_server_ready(&addr);

        let addr_parts: Vec<&str> = addr.split(':').collect();
        let host = addr_parts[0];
        let port: u16 = addr_parts[1].parse().expect("Invalid port");

        let mut client = FtpClient::new(host, port, "anonymous", "", "test_user");
        client.connect().expect("Failed to connect to FTP server");

        // Generate unique root and nested directories
        let uuid = Uuid::new_v4();
        let root_dir = format!("upload1/test_del_{}", uuid);
        let sub_dir = format!("{}/nested", root_dir);

        // Create directories
        client.make_directory("upload1").ok();
        client.make_directory(&root_dir).expect("Could not create root dir");
        client.make_directory(&sub_dir).expect("Could not create nested dir");

        // Create local temp files
        let temp_dir = std::env::temp_dir();
        let file1_path = temp_dir.join(format!("ftp_temp_root_{}.txt", uuid));
        let file2_path = temp_dir.join(format!("ftp_temp_nested_{}.txt", uuid));

        std::fs::write(&file1_path, "Root level file\n").expect("Failed to write temp file1");
        std::fs::write(&file2_path, "Nested file\n").expect("Failed to write temp file2");

        // Upload files to FTP server
        let remote1 = format!("{}/file1.txt", root_dir);
        let remote2 = format!("{}/file2.txt", sub_dir);
        client.upload_file(file1_path.to_str().unwrap(), &remote1).expect("Upload file1 failed");
        client.upload_file(file2_path.to_str().unwrap(), &remote2).expect("Upload file2 failed");

        println!("Files uploaded to test directories");

        // ✅ Recursive deletion from inside /test_user
        client.remove_directory_recursive(&root_dir).expect("Recursive deletion failed");
        println!("Recursive deletion complete for {}", root_dir);

        // Clean up local temp files
        let _ = std::fs::remove_file(&file1_path);
        let _ = std::fs::remove_file(&file2_path);

        client.disconnect();
        println!("Disconnected");

        shutdown_tx.send(()).expect("Failed to send shutdown");
        handle.join().expect("Server thread panicked");
    }

}
