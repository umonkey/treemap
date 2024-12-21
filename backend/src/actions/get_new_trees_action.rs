use crate::services::AppState;
use crate::types::{AddedTreesRequest, Result, TreeList};
use actix_web::{get, web::Data, web::Json, web::Query};

#[get("/v1/trees/new")]
pub async fn get_new_trees_action(
    state: Data<AppState>,
    query: Query<AddedTreesRequest>,
) -> Result<Json<TreeList>> {
    let count = query.get_count();
    let skip = query.get_skip();
    let trees = state.get_new_trees_handler.handle(count, skip).await?;
    Ok(Json(trees))
}
