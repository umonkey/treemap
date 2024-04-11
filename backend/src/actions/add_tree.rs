use actix_web::{post, web::Data, web::Json, HttpRequest};
use serde::Deserialize;

use crate::Result;
use crate::services::app::AppState;
use crate::types::{TreeInfo, AddTreeRequest};

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub lat: f64,
    pub lon: f64,
    pub name: String,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
}

#[post("/v1/trees")]
pub async fn add_tree(
    state: Data<AppState>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeInfo>> {
    let user_id = state.get_user_id(&req)?;

    let req = AddTreeRequest {
        lat: payload.lat,
        lon: payload.lon,
        name: payload.name.clone(),
        height: payload.height,
        circumference: payload.circumference,
        diameter: payload.diameter,
        state: payload.state.clone(),
        user_id,
    };

    let tree = state.add_tree(req).await?;
    Ok(Json(tree))
}
