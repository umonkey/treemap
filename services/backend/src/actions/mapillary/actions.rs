use crate::actions::mapillary::schemas::GetMapillaryRequest;
use crate::domain::mapillary::MapillaryService;
use crate::domain::tree::Bounds;
use crate::responders::geo_json::respond_with_mapillary;
use crate::services::*;
use crate::types::*;
use actix_web::web::{Data, Path, Query};
use actix_web::HttpResponse;

pub async fn get_mapillary_geo_json_action(
    state: Data<AppState>,
    query: Query<GetMapillaryRequest>,
) -> Result<HttpResponse> {
    let service = state.build::<MapillaryService>()?;
    let bounds = Bounds {
        n: query.n,
        e: query.e,
        s: query.s,
        w: query.w,
    };

    let mut images = Vec::new();
    if query.points {
        images = service.get_images_by_bounds(bounds.clone()).await?;
    }

    let mut sequences = Vec::new();
    if query.lines {
        sequences = service.get_sequences_by_bounds(bounds).await?;
    }

    Ok(respond_with_mapillary(&images, &sequences))
}

pub async fn get_mapillary_image_action(
    state: Data<AppState>,
    path: Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let service = state.build::<MapillaryService>()?;
    let image = service.get_image_metadata(&id).await?;

    Ok(HttpResponse::Ok().json(image))
}
