use actix_web::{get, web::Data, web::Json, web::Query};
use log::debug;
use serde::Deserialize;

use crate::services::app::AppState;
use crate::types::{Bounds, TreeList};
use crate::Result;

#[derive(Deserialize)]
struct QueryParams {
    n: f64,
    e: f64,
    s: f64,
    w: f64,
}

#[get("/v1/trees")]
pub async fn get_trees(state: Data<AppState>, query: Query<QueryParams>) -> Result<Json<TreeList>> {
    let bounds = Bounds {
        n: query.n,
        e: query.e,
        s: query.s,
        w: query.w,
    };

    let trees = state.get_trees(bounds).await?;

    debug!(
        "Returning {} trees for n={} e={} s={} w={}",
        trees.len(),
        query.n,
        query.e,
        query.s,
        query.w
    );

    Ok(Json(trees))
}
