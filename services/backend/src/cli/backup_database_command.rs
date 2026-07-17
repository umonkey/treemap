use crate::services::AppState;
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::Read;
use tempfile::NamedTempFile;

pub async fn backup_database_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let temp_path = temp_file.path().to_str().expect("Failed to get temp path");

    // Perform VACUUM INTO to the temporary file.
    // This creates a consistent copy of the database.
    state
        .database
        .execute_sql(&format!("VACUUM INTO '{}'", temp_path), &[])
        .await
        .expect("Failed to vacuum database into temp file");

    // Read the temp file.
    let mut file = std::fs::File::open(temp_path).expect("Failed to open temp file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read temp file");

    // Compress the data.
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&buffer)
        .expect("Failed to compress database");
    let compressed_data = encoder.finish().expect("Failed to finish compression");

    // Upload the compressed data.
    let date = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("{}.sqlite.gz", date);

    state
        .backups
        .write_file(&filename, &compressed_data, false)
        .await
        .expect("Failed to upload backup");

    println!("Database backup uploaded as {}", filename);
}
