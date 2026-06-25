use crate::actions::mapillary::schemas::{
    AddMapillaryTreeRequest, GetMapillaryRequest, ReplaceMapillaryTreesRequest,
};
use crate::domain::mapillary::{
    MapillarySequenceDetail, MapillarySequenceSummary, MapillaryService, MapillaryTree,
    UpdateMapillarySequence,
};
use crate::domain::tree::Bounds;
use crate::responders::geo_json::respond_with_mapillary;
use crate::services::app::{PanoEdit, RequirePermission};
use crate::services::*;
use crate::types::*;
use actix_web::web::{Data, Json, Path, Query};
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

pub async fn get_mapillary_hints_action(
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

    let geojson = service.get_tree_hints_geojson(bounds).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/geo+json")
        .json(geojson))
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

pub async fn get_mapillary_image_trees_action(
    state: Data<AppState>,
    path: Path<String>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let service = state.build::<MapillaryService>()?;
    let trees = service.get_image_trees(&id).await?;

    Ok(HttpResponse::Ok().json(trees))
}

pub async fn get_mapillary_sequences_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
) -> Result<Json<Vec<MapillarySequenceSummary>>> {
    let sequences = service.get_all_sequences().await?;
    Ok(Json(sequences))
}

pub async fn get_mapillary_sequence_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
) -> Result<Json<MapillarySequenceDetail>> {
    let id = path.into_inner();
    let sequence = service.get_sequence_detail(&id).await?;
    Ok(Json(sequence))
}

pub async fn update_mapillary_sequence_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
    body: Json<UpdateMapillarySequence>,
) -> Result<HttpResponse> {
    let id = path.into_inner();
    service.update_sequence(&id, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn add_mapillary_image_tree_action(
    user_id: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
    body: Json<AddMapillaryTreeRequest>,
) -> Result<HttpResponse> {
    let tree = MapillaryTree {
        image_id: path.into_inner(),
        angle: body.angle,
        tree_id: body.tree_id,
        user_id: *user_id,
    };

    service.add_image_tree(tree).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn delete_mapillary_image_trees_action(
    _user: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
) -> Result<HttpResponse> {
    service.delete_image_trees(&path.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn replace_mapillary_image_trees_action(
    user_id: RequirePermission<PanoEdit>,
    service: Injected<MapillaryService>,
    path: Path<String>,
    body: Json<ReplaceMapillaryTreesRequest>,
) -> Result<HttpResponse> {
    let image_id = path.into_inner();

    let trees = body
        .trees
        .iter()
        .map(|t| MapillaryTree {
            image_id: image_id.clone(),
            angle: t.angle,
            tree_id: t.tree_id,
            user_id: *user_id,
        })
        .collect();

    service.replace_image_trees(&image_id, trees).await?;

    Ok(HttpResponse::NoContent().finish())
}
