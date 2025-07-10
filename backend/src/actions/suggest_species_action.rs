//! Suggest species action.
//! This returns species suggested for the user, based on their recently added trees.

use crate::services::AppState;
use crate::types::Result;
use actix_web::{get, web::Data, web::Json, HttpRequest};

#[get("/v1/species/suggest")]
pub async fn suggest_species_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<Vec<String>>> {
    let user_id = state.get_user_id(&req)?;
    let species = state.suggest_species_handler.handle(user_id).await?;

    Ok(Json(species))
}
