//! This endpoint receives requests to replace a tree with another one.

use crate::services::AppState;
use crate::types::{ReplaceTreeRequest, Result, TreeList};
use actix_web::{put, web::Data, web::Json, web::Path, HttpRequest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub year: Option<i64>,
}

#[put("/v1/trees/{id}/replace")]
pub async fn replace_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let trees = state
        .replace_tree_handler
        .handle(ReplaceTreeRequest {
            id: path.id,
            user_id: state.get_user_id(&req)?,
            species: payload.species.clone(),
            notes: payload.notes.clone(),
            height: payload.height,
            circumference: payload.circumference,
            diameter: payload.diameter,
            state: payload.state.clone(),
            year: payload.year,
        })
        .await?;

    Ok(Json(TreeList::from_trees(&trees)))
}
