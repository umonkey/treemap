use actix_web::{get, web::Data, web::Json, web::Query};
use log::debug;
use serde::Deserialize;

use crate::Result;
use crate::objects::{Bounds, TreeList};
use crate::services::app::AppState;

#[derive(Deserialize)]
struct QueryParams {
    n: f64,
    e: f64,
    s: f64,
    w: f64,
}

#[get("/v1/trees")]
pub async fn get_trees(
    state: Data<AppState>,
    query: Query<QueryParams>,
) -> Result<Json<TreeList>> {
    let bounds = Bounds {
        n: query.n,
        e: query.e,
        s: query.s,
        w: query.w,
    };

    let trees = state.get_trees(bounds).await?;

    debug!("Returning {} trees", trees.len());

    Ok(Json(trees))
}
