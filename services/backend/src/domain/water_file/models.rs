//! Link rows between water sources and files.

use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct WaterFile {
    pub id: u64,
    pub water_source_id: u64,
    pub file_id: u64,
    pub added_at: u64,
    pub added_by: u64,
}
