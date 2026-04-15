use crate::domain::tree::{DuplicatesResponse, TreeService};
use crate::services::{AppState, ContextExt};
use crate::types::Result;
use actix_web::get;
use actix_web::web::{Data, Json, ServiceConfig};

#[get("")]
pub async fn get_duplicates_action(state: Data<AppState>) -> Result<Json<DuplicatesResponse>> {
    let duplicates = state.build::<TreeService>()?.get_duplicates().await?;
    Ok(Json(duplicates))
}

// Configure the router.
pub fn duplicate_router(cfg: &mut ServiceConfig) {
    cfg.service(get_duplicates_action);
}
