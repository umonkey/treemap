use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExportFile {
    pub name: String,
    pub url: String,
    pub size: u64,
}
