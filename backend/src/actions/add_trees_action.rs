//! This endpoint receives requests to add one new tree.

use crate::services::AppState;
use crate::types::{AddTreeRequest, LatLon, Result, TreeList};
use actix_web::{post, web::Data, web::Json, HttpRequest};
use serde::Deserialize;

fn default_state() -> String {
    "unknown".to_string()
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub points: Vec<LatLon>,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    #[serde(default = "default_state")]
    pub state: String,
    pub year: Option<i64>,
    pub address: Option<String>,
}

#[post("/v1/trees")]
pub async fn add_trees_action(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let user_id = state.get_user_id(&req)?;

    let trees = state
        .add_trees_handler
        .handle(AddTreeRequest {
            points: payload.points.clone(),
            species: payload.species.clone(),
            notes: payload.notes.clone(),
            height: payload.height,
            circumference: payload.circumference,
            diameter: payload.diameter,
            state: payload.state.clone(),
            user_id,
            year: payload.year,
            address: payload.address.clone(),
        })
        .await?;

    Ok(Json(TreeList::from_trees(&trees)))
}
