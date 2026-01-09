use crate::services::AppState;
use crate::types::*;
use actix_web::web::ServiceConfig;
use actix_web::{get, web::Data, web::Json, web::Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("/v1/stats/top-users")]
pub async fn get_top_users(state: Data<AppState>) -> Result<Json<UserList>> {
    let res = state.user_service.get_top_users().await?;
    Ok(Json(res))
}

#[get("/{id}")]
pub async fn get_user(state: Data<AppState>, path: Path<PathInfo>) -> Result<Json<UserResponse>> {
    let user = state.user_service.get_user(path.id).await?;
    Ok(Json(user))
}

#[get("/{id}/heatmap")]
pub async fn get_user_heatmap(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<Vec<HeatmapResponse>>> {
    let stats = state.get_heatmap_handler.handle_user(path.id).await?;
    Ok(Json(stats))
}

// Configure the router.
pub fn users_router(cfg: &mut ServiceConfig) {
    cfg.service(get_user_heatmap).service(get_user);
}
