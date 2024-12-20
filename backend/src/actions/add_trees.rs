use actix_web::{post, web::Data, web::Json, HttpRequest};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{AddTreeRequest, LatLon, Result, TreeList};

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub points: Vec<LatLon>,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub year: Option<i64>,
}

#[post("/v1/trees")]
pub async fn add_trees(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let user_id = state.get_user_id(&req)?;

    let req = AddTreeRequest {
        points: payload.points.clone(),
        species: payload.species.clone(),
        notes: payload.notes.clone(),
        height: payload.height,
        circumference: payload.circumference,
        diameter: payload.diameter,
        state: payload.state.clone(),
        user_id,
        year: payload.year,
    };

    let trees = state.add_trees(req).await?;
    Ok(Json(TreeList::from_trees(trees)))
}
