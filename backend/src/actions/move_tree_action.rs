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
    pub lat: f64,
    pub lon: f64,
}

#[put("/v1/trees/{id}/position")]
pub async fn move_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .move_tree_handler
        .handle(MoveTreeRequest {
            id: path.id,
            lat: payload.lat,
            lon: payload.lon,
            user_id,
        })
        .await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}
