//! Receive a binary file from the user, attach to a tree.

use crate::services::AppState;
use crate::types::*;
use actix_web::{post, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct RequestPayload {
    pub files: Vec<String>,
}

#[post("/v1/trees/{id}/photos")]
pub async fn add_photos_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    let req = AddPhotosRequest {
        tree_id: path.id,
        files: payload.files.clone(),
        user_id,
    };

    state.add_photos_handler.handle(req).await?;

    Ok(HttpResponse::Accepted().finish())
}
