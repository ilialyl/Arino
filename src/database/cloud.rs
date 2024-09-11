use reqwest::Client;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io;
use std::io::Write;
use std::time::Duration;
use std::collections::HashMap;
use serde::Deserialize;

pub async fn sync() -> Result<(), Box<dyn std::error::Error>> {
    // Your Dropbox access token
    let access_token = match get_access_token().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    };

    // The file you want to upload
    let file_path = "database.db";
    let destination_path = "/database.db"; // Where to upload in Dropbox

    // Read the file content
    let mut file = File::open(file_path)?;
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content)?;

    // Set up the request client
    let client = Client::new();

    // Send the file to Dropbox
    let response = client
        .post("https://content.dropboxapi.com/2/files/upload")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Dropbox-API-Arg", format!(r#"{{"path": "{}","mode": "add","autorename": true,"mute": false,"strict_conflict": false}}"#, destination_path))
        .header("Content-Type", "application/octet-stream")
        .body(file_content)
        .send()
        .await?;

    // Check if the upload was successful
    if response.status().is_success() {
        println!("Database synced successfully");
    } else {
        let error_message = response.text().await?;
        println!("Failed to sync database: {}", error_message);
    }

    Ok(())
}

pub async fn fetch(source: Database) -> Result<(), Box<dyn std::error::Error>> {
    // Your Dropbox access token
    let access_token = match get_access_token().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return Ok(());
        },
    };

    let dropbox_path = match source {
        Database::Main => "/database.db",
        Database::Backup => "/database_backup.db"
    };

    // Set up the request client
    let client = Client::new();

    // Send the download request
    let response = client
        .post("https://content.dropboxapi.com/2/files/download")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Dropbox-API-Arg", format!(r#"{{"path": "{}"}}"#, dropbox_path))
        .send()
        .await?;

    // Check if the download was successful
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

pub async fn has_internet_access() -> bool {
    let client = Client::new();
    let url = "https://www.google.com";

    match client.get(url).timeout(Duration::from_secs(5)).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}


#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
}

async fn get_access_token() -> Result<String, Box<dyn std::error::Error>> {
    let creditials = match get_creditials() {
        Ok(c) => c,
        Err(_) => {
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "Creditials not found")));
        },
    };

    let client_id = creditials.client_id;
    let client_secret = creditials.client_secret;
    let refresh_token = creditials.refresh_token;
    
    // Dropbox token endpoint
    let token_url = "https://api.dropboxapi.com/oauth2/token";

    // Prepare form data for the request
    let mut params = HashMap::new();
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token".to_string());
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);

    // Create an HTTP client
    let client = Client::new();

    // Send the request to Dropbox API
    let response = client
        .post(token_url)
        .form(&params)
        .send()
        .await?;

    // Handle response based on status code
    let status = response.status();
    if status.is_success() {
        let token_response: TokenResponse = response.json().await?;
        Ok(token_response.access_token)
    } else {
        // Get the error text from the response
        let error_text = response.text().await?;
        eprintln!("Failed to get access token: {}", error_text);
        Err(format!("Failed request with status: {}", status).into())
    }
}

#[derive(Deserialize)]
struct Creditials {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

fn get_creditials() -> Result<Creditials, Box<dyn std::error::Error>> {
    let json_string = fs::read_to_string("key.json");
    let json_string = match json_string {
        Ok(s) => s,
        Err(_) => {
            return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, "key not found")));
        },
    };
    let parsed: Creditials = serde_json::from_str(&json_string).unwrap();

    Ok(parsed)
}

pub enum Database {
    Main,
    Backup
}