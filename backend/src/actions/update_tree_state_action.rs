use crate::services::AppState;
use crate::types::*;
use actix_web::{put, web::Data, web::Json, web::Path, HttpRequest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub value: String,
    pub comment: Option<String>,
}

#[put("/v1/trees/{id}/state")]
pub async fn update_tree_state_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;

    let tree = state
        .update_tree_state_handler
        .handle(
            path.id,
            payload.value.clone(),
            user_id,
            payload.comment.clone(),
        )
        .await?;

    Ok(Json(tree))
}
