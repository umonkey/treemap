use actix_web::{get, web::Data, web::Json};

use crate::Result;
use crate::objects::TreeList;
use crate::services::app::AppState;

#[get("/v1/trees")]
pub async fn get_trees(
    state: Data<AppState>
) -> Result<Json<TreeList>> {
    let trees = state.get_trees().await?;

    Ok(Json(trees))
}
