use actix_web::{get, web::Data, web::Json};

use crate::services::AppState;
use crate::types::{Result, TreeStats};

#[get("/v1/trees/stats")]
pub async fn get_tree_stats(state: Data<AppState>) -> Result<Json<TreeStats>> {
    let stats = state.get_tree_stats().await?;

    Ok(Json(stats))
}
