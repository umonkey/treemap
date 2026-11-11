use crate::services::AppState;
use crate::types::{Error, Result};
use chrono::Local;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::io::Read;
use tempfile::NamedTempFile;

pub async fn backup_database_command() -> Result<()> {
    let state = AppState::new().await?;

    let temp_file = NamedTempFile::new()?;
    let temp_path = temp_file
        .path()
        .to_str()
        .ok_or_else(|| Error::Io("temporary backup path is not valid UTF-8".to_string()))?;

    // Perform VACUUM INTO to the temporary file.
    // This creates a consistent copy of the database.
    state
        .database
        .execute_sql(&format!("VACUUM INTO '{}'", temp_path), &[])
        .await?;

    // Read the temp file.
    let mut file = std::fs::File::open(temp_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Compress the data.
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&buffer)?;
    let compressed_data = encoder.finish()?;

    // Upload the compressed data.
    let date = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("{}.sqlite.gz", date);

    state
        .backups
        .write_file(&filename, &compressed_data, false)
        .await?;

    println!("Database backup uploaded as {}", filename);

    Ok(())
}
