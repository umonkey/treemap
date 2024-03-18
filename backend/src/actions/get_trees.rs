use actix_web::{get, web::Data, web::Json};
use log::debug;

use crate::Result;
use crate::objects::TreeList;
use crate::services::app::AppState;

#[get("/v1/trees")]
pub async fn get_trees(
    state: Data<AppState>
) -> Result<Json<TreeList>> {
    let trees = state.get_trees().await?;

    debug!("Returning {} trees", trees.len());

    Ok(Json(trees))
}
