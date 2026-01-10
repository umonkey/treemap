use crate::services::AppState;
use crate::types::{DuplicatesResponse, Result};
use actix_web::get;
use actix_web::web::{Data, Json, ServiceConfig};

#[get("/v1/duplicates")]
pub async fn get_duplicates_action(state: Data<AppState>) -> Result<Json<DuplicatesResponse>> {
    let duplicates = state.trees.get_duplicates().await?;
    Ok(Json(duplicates))
}

// Configure the router.
pub fn duplicate_router(cfg: &mut ServiceConfig) {
    cfg.service(get_duplicates_action);
}
