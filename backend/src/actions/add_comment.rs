/**
 * Receive a comment from the user, attach to a tree.
 */
use actix_web::{post, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{AddCommentRequest, Result};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub message: String,
}

#[post("/v1/trees/{id}/comments")]
pub async fn add_comment(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    let req = AddCommentRequest {
        tree_id: path.id,
        message: payload.message.to_string(),
        user_id,
    };

    state.add_comment(req).await?;

    Ok(HttpResponse::Accepted().finish())
}
