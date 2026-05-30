use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MapillaryImage {
    pub id: String,
    pub sequence: String,
    pub captured_at: i64,
    pub geometry: MapillaryGeometry,
    pub compass_angle: Option<f64>,
    pub is_pano: bool,
    pub quality_score: Option<f64>,
    pub thumb_2048_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MapillaryGeometry {
    pub coordinates: [f64; 2],
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MapillaryResponse {
    pub data: Vec<MapillaryImage>,
    pub paging: Option<MapillaryPaging>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MapillaryPaging {
    pub next: String,
}
