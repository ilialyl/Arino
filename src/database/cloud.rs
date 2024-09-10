use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub async fn upload() -> Result<(), Box<dyn std::error::Error>> {
    // Your Dropbox access token
    let access_token = "sl.B8kaY9bwoLGaveJqeo-BCqQbXLfthxyCNGga60_LmWho0Kql-spElGVLaCJS3RlZsF2vIuNKbb0Abm7LuXB_lkpco9ppnQXAG6JbQ2QtIYy8HoSaoKvirSAEgGslx8s-ktTcOFN7wBbpHRM";

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
        println!("File uploaded successfully!");
    } else {
        let error_message = response.text().await?;
        println!("Failed to upload file: {}", error_message);
    }

    Ok(())
}

pub async fn fetch(source: Database) -> Result<(), Box<dyn std::error::Error>> {
    // Your Dropbox access token
    let access_token = "sl.B8kaY9bwoLGaveJqeo-BCqQbXLfthxyCNGga60_LmWho0Kql-spElGVLaCJS3RlZsF2vIuNKbb0Abm7LuXB_lkpco9ppnQXAG6JbQ2QtIYy8HoSaoKvirSAEgGslx8s-ktTcOFN7wBbpHRM";

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

pub enum Database {
    Main,
    Backup
}