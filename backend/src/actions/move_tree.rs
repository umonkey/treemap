use actix_web::{put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

use crate::services::app::AppState;
use crate::types::MoveTreeRequest;
use crate::Result;

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
pub async fn move_tree(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    let req = MoveTreeRequest {
        id: path.id,
        lat: payload.lat,
        lon: payload.lon,
        user_id,
    };

    state.move_tree(req).await?;

    Ok(HttpResponse::Accepted()
       .content_type("application/json")
       .finish())
}
