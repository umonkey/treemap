use crate::services::AppState;
use crate::types::*;
use actix_web::{put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub file: String,
}

#[put("/v1/trees/{id}/thumbnail")]
pub async fn update_tree_thumbnail_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    let req = UpdateTreeThumbnailRequest {
        tree_id: path.id,
        file_id: payload.file.parse().map_err(|_| Error::BadImage)?,
        user_id,
    };

    state.update_tree_thumbnail_handler.handle(req).await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}
