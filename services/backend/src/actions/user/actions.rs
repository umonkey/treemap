use super::schemas::{UserList, UserRead};
use crate::domain::heatmap::{HeatmapItem, HeatmapService};
use crate::domain::user::{UserService, UserUpdate};
use crate::services::AppState;
use crate::services::Injected;
use crate::types::*;
use actix_web::web::ServiceConfig;
use actix_web::{get, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[get("")]
pub async fn get_users(user_service: Injected<UserService>) -> Result<Json<UserList>> {
    let res = user_service.list().await?;
    Ok(Json(res.into()))
}

#[get("/{id}")]
pub async fn get_user(
    user_service: Injected<UserService>,
    path: Path<PathInfo>,
) -> Result<Json<UserRead>> {
    let user = user_service.get_user(path.id).await?;
    Ok(Json(user.into()))
}

#[get("/{id}/heatmap")]
pub async fn get_user_heatmap(
    heatmap_service: Injected<HeatmapService>,
    path: Path<PathInfo>,
) -> Result<Json<Vec<HeatmapItem>>> {
    let stats = heatmap_service.get_user(path.id).await?;
    Ok(Json(stats))
}

#[put("/{id}")]
pub async fn update_user_action(
    state: Data<AppState>,
    user_service: Injected<UserService>,
    path: Path<PathInfo>,
    body: Json<UserUpdate>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let current_user_id = state.get_user_id(&req)?;
    user_service
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
