use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

use crate::services::AppState;
use crate::types::{Result, UserResponse};

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/users/{id}")]
pub async fn get_user_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<UserResponse>> {
    let user = state.get_user_handler.handle(path.id).await?;
    Ok(Json(user))
}
