use actix_web::{post, web::Data, web::Json};

use crate::Result;
use crate::objects::{TreeInfo, AddTreeRequest};
use crate::services::app::AppState;

#[post("/v1/trees")]
pub async fn add_tree(
    state: Data<AppState>,
    payload: Json<AddTreeRequest>,
) -> Result<Json<TreeInfo>> {
    let req = AddTreeRequest {
        lat: payload.lat,
        lon: payload.lon,
        name: payload.name.clone(),
    };

    let tree = state.add_tree(req).await?;
    Ok(Json(tree))
}
