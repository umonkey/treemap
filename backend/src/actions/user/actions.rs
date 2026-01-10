use super::schemas::{UserList, UserRead};
use crate::domain::heatmap::HeatmapItem;
use crate::domain::user::UserUpdate;
use crate::services::AppState;
use crate::types::*;
use actix_web::web::ServiceConfig;
use actix_web::{get, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("")]
pub async fn get_users(state: Data<AppState>) -> Result<Json<UserList>> {
    let res = state.users.list().await?;
    Ok(Json(res.into()))
}

#[get("/{id}")]
pub async fn get_user(state: Data<AppState>, path: Path<PathInfo>) -> Result<Json<UserRead>> {
    let user = state.users.get_user(path.id).await?;
    Ok(Json(user.into()))
}

#[get("/{id}/heatmap")]
pub async fn get_user_heatmap(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<Vec<HeatmapItem>>> {
    let stats = state.heatmap.get_user(path.id).await?;
    Ok(Json(stats))
}

#[put("/{id}")]
pub async fn update_user_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    body: Json<UserUpdate>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user_id = state.get_user_id(&req)?;
    state
        .users
        .update_user(current_user_id, path.id, body.into_inner())
        .await?;

    Ok(HttpResponse::Accepted().finish())
}

// Configure the router.
pub fn user_router(cfg: &mut ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_heatmap)
        .service(get_user)
        .service(update_user_action);
}
