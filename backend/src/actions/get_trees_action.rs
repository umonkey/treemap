use actix_web::{get, web::Data, web::Json, web::Query, HttpRequest};
use log::debug;

use crate::services::AppState;
use crate::types::{GetTreesRequest, Result, TreeList};

#[get("/v1/trees")]
pub async fn get_trees_action(
    state: Data<AppState>,
    query: Query<GetTreesRequest>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let user_id = state.get_user_id(&req).unwrap_or(0);
    let trees = state.get_trees_handler.handle(&query, user_id).await?;

    debug!("Returning {} trees.", trees.len());

    Ok(Json(trees))
}
