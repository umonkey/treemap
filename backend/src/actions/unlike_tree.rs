use crate::services::AppState;
use crate::types::Result;
use actix_web::{delete, web::Data, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub tree_id: u64,
}

#[delete("/v1/trees/{tree_id}/likes")]
pub async fn unlike_tree(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;
    state.unlike_tree(path.tree_id, user_id).await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}
