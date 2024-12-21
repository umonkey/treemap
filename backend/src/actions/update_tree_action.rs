use actix_web::{put, web::Data, web::Json, web::Path, HttpRequest};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{Result, TreeRecord, UpdateTreeRequest};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub species: Option<String>,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: Option<String>,
    pub year: Option<i64>,
}

#[put("/v1/trees/{id}")]
pub async fn update_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeRecord>> {
    let user_id = state.get_user_id(&req)?;

    let tree = state
        .update_tree_handler
        .handle(UpdateTreeRequest {
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
            year: payload.year,
        })
        .await?;

    Ok(Json(tree))
}
