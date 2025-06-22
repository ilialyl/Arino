use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::time::Duration;

use crate::client::has_access;
use crate::database::database_exists;
use crate::miscellaneous::flush;

// Database sync types
pub enum Database {
    Main,
    Backup,
}

// Token Response struct containing a string of access token.
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

// Credentials struct containing client id, secret key, and refresh token.
#[derive(Deserialize)]
pub struct Credentials {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

// Pushes the database to Cloud as main
pub async fn push() -> Result<(), Box<dyn std::error::Error>> {
    if !has_access() {
        return Ok(());
    }
    // Checks if the access token is valid, and refreshes it if necessary
    if !check_access_token_validity().await? {
        request_access_token().await?;
    }

    // Retrieves the access token
    let access_token = match retrieve_access_token() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let file_path = "database.db";
    let destination_path = "/database.db";

    // Reads the file content
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Sets up the request client
    let client = Client::new();

    // Sends the file to Dropbox with overwrite mode
    let response = client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header("Authorization", format!("Bearer {}", access_token))
        .header(
            "Dropbox-API-Arg",
            format!(
                r#"{{"path": "{}","mode": "overwrite","autorename": false,"mute": false}}"#,
                destination_path
            ),
        )
        .header("Content-Type", "application/octet-stream")
        .body(file_content)
        .send()
        .await?;

    // Check if the upload was successful
    if response.status().is_success() {
        println!("Database synced successfully");
    } else {
        let error_message = response.text().await?;
        eprintln!("Failed to sync database: {}", error_message);
    }

    Ok(())
}

// Pushes the database to cloud as a backup
pub async fn backup() -> Result<(), Box<dyn std::error::Error>> {
    // Checks if the access token is valid, and refreshes it if necessary
    if !check_access_token_validity().await? {
        request_access_token().await?;
    }

    let access_token = match retrieve_access_token() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    // Paths to files
    let file_path = "database.db";
    let destination_path = "/database_backup.db"; // Where to upload in Dropbox

    // Reads the file content
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Sets up the request client
    let client = Client::new();

    // Sends the file to Dropbox
    let response = client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Dropbox-API-Arg", format!(r#"{{"path": "{}","mode": "add","autorename": true,"mute": false,"strict_conflict": false}}"#, destination_path))
        .header("Content-Type", "application/octet-stream")
        .body(file_content)
        .send()
        .await?;

    // Checks if the upload was successful
    if response.status().is_success() {
        println!("Database backed up successfully");
    } else {
        let error_message = response.text().await?;
        println!("Database backup failed: {}", error_message);
    }

    Ok(())
}

// Fetches the database file from Cloud.
pub async fn fetch(source: Database) -> Result<(), Box<dyn std::error::Error>> {
    if !has_access() {
        return Ok(());
    }

    // Checks if the access token is valid, and refreshes it if necessary
    if !check_access_token_validity().await? {
        request_access_token().await?;
    }
    let access_token = match retrieve_access_token() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        }
    };

    let dropbox_path = match source {
        Database::Main => "/database.db",
        Database::Backup => "/database_backup.db",
    };

    // Sets up the request client
    let client = Client::new();

    // Sends the download request
    let response = client
        .post("https://content.dropboxapi.com/2/files/download")
        .header("Authorization", format!("Bearer {}", access_token))
        .header(
            "Dropbox-API-Arg",
            format!(r#"{{"path": "{}"}}"#, dropbox_path),
        )
        .send()
        .await?;

    // Checks if the download was successful
    if response.status().is_success() {
        let file_content = response.bytes().await?;
        let mut file = File::create("database.db")?;
        file.write_all(&file_content)?;
        println!("Database fetched successfully");
    } else {
        let error_message = response.text().await?;
        println!("Failed to fetch the database: {}", error_message);
    }

    Ok(())
}

// Checks if the client has internet access.
pub async fn has_internet_access() -> bool {
    if database_exists() & !has_access() {
        return true;
    }

    let client = Client::new();
    let url = "https://www.google.com";

    match client.get(url).timeout(Duration::from_secs(5)).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => {
            println!("Error: Internet access is needed");
            false
        }
    }
}

// Requests access token from Dropbox using client id and secret key.
async fn request_access_token() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = match get_credentials() {
        Ok(c) => c,
        Err(e) => {
            eprint!("Creditial not found: {e}");
            return Ok(());
        }
    };

    let client_id = credentials.client_id;
    let client_secret = credentials.client_secret;
    let refresh_token = credentials.refresh_token;

    // Dropbox token endpoint
    let token_url = "https://api.dropboxapi.com/oauth2/token";

    // Prepares form data for the request
    let mut params = HashMap::new();
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token".to_string());
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);

    // Creates an HTTP client
    let client = Client::new();

    // Sends the request to Dropbox API
    let response = client.post(token_url).form(&params).send().await?;

    // Handles response based on status code
    let status = response.status();
    if status.is_success() {
        let token_response: TokenResponse = response.json().await?;
        store_access_token(token_response.access_token);
    } else {
        // Gets the error text from the response
        let error_text = response.text().await?;
        eprintln!("Failed to request access token: {}", error_text);
    }
    Ok(())
}

// Stores access token in a json file
fn store_access_token(access_token: String) {
    let json_string = match serde_json::to_string(&access_token) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error storing parsing access token to json: {e}");
            return;
        }
    };

    match std::fs::write("access_token.json", json_string) {
        Ok(_) => {}
        Err(e) => eprintln!("Error storing access token: {e}"),
    }
}

// Retrieves access token from a json file
fn retrieve_access_token() -> Result<String, Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string("access_token.json");
    let json_string = match json_string {
        Ok(s) => s,
        Err(_) => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "stored access token not found",
            )));
        }
    };
    let parsed: String = serde_json::from_str(&json_string).unwrap();

    Ok(parsed)
}

// Checks if the current access token can still be used.
async fn check_access_token_validity() -> Result<bool, Box<dyn std::error::Error>> {
    let access_token = match retrieve_access_token() {
        Ok(s) => s,
        Err(_) => {
            eprint!("Using new access token...");
            flush();
            print!("\r");
            return Ok(false);
        }
    };

    // Dropbox check endpoint (this is a lightweight request)
    let url = "https://api.dropboxapi.com/2/users/get_current_account";

    // Creates an HTTP client
    let client = Client::new();

    // Sends the request with the access token
    let response = client
        .post(url)
        .bearer_auth(access_token) // Use the access token for authorization
        .send()
        .await?;

    // Checks if the response status is successful (2xx)
    if response.status().is_success() {
        eprint!("Using old access token...");
        flush();
        print!("\r");
        Ok(true) // Token is valid
    } else if response.status().as_u16() == 401 {
        eprint!("Using new access token...");
        flush();
        print!("\r");
        Ok(false) // Token is invalid or expired
    } else {
        eprintln!("Unexpected error");
        Err(format!("Unexpected error: {}", response.status()).into())
    }
}

// Retrieves credientials from a file.
pub fn get_credentials() -> Result<Credentials, Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string("dropbox_credentials.json");
    let json_string = match json_string {
        Ok(s) => s,
        Err(_) => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "key not found",
            )));
        }
    };
    let parsed: Credentials = serde_json::from_str(&json_string).unwrap();

    Ok(parsed)
}

pub async fn download_database() {
    let response = reqwest::get(
        "https://www.dropbox.com/scl/fi/nfjsio3pr33ppwzp0c46u/database.db?rlkey=z84k05kfed2clj5ri9621q57w&st=94yvl88v&dl=1",
    ).await.expect("Download failed");
    let db = response.bytes().await.expect("File invalid");

    let mut file = File::create("database.db").expect("Failed to create file");
    file.write_all(&db).expect("Failed to write into file");
}
