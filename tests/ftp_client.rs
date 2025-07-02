use chiral_client::ftp::FtpClient;
use std::fs::{self, File};
use std::io::Write;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

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


