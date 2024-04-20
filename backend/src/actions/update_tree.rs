use actix_web::{put, web::Data, web::Json, web::Path, HttpRequest};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{Result, TreeInfo, UpdateTreeRequest};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub lat: f64,
    pub lon: f64,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
}

#[put("/v1/trees/{id}")]
pub async fn update_tree(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeInfo>> {
    let user_id = state.get_user_id(&req)?;

    let req = UpdateTreeRequest {
        id: path.id,
        lat: payload.lat,
        lon: payload.lon,
        species: payload.species.clone(),
        notes: payload.notes.clone(),
        height: payload.height,
        circumference: payload.circumference,
        diameter: payload.diameter,
        state: payload.state.clone(),
        user_id,
    };

    let tree = state.update_tree(req).await?;
    Ok(Json(tree))
}
