/**
 * This is how a single tree is stored in the database.
 */
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct TreeInfo {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub added_at: u64,
    pub updated_at: u64,
    pub added_by: u64,
    pub thumbnail_id: Option<u64>,
}
