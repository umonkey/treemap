use crate::services::AppState;
use crate::types::{AddedTreesRequest, Result, TreeList};
/**
 * Returns a list of recently added trees.
 */
use actix_web::{get, web::Data, web::Json, web::Query};
use log::debug;

#[get("/v1/trees/new")]
pub async fn get_new_trees(
    state: Data<AppState>,
    query: Query<AddedTreesRequest>,
) -> Result<Json<TreeList>> {
    let trees = state
        .get_new_trees(query.get_count(), query.get_skip())
        .await?;

    debug!("Returning {} trees.", trees.len());

    Ok(Json(trees))
}
