use actix_web::{get, web::Data, web::Json, web::Query};
use log::debug;

use crate::services::AppState;
use crate::types::{GetTreesRequest, Result, TreeList};

#[get("/v1/trees")]
pub async fn get_trees(state: Data<AppState>, query: Query<GetTreesRequest>) -> Result<Json<TreeList>> {
    let trees = state.get_trees(&query).await?;

    debug!("Returning {} trees.", trees.len());

    Ok(Json(trees))
}
