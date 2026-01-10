use super::schemas::AddFileRequest;
use super::schemas::AddPhotosRequest;
use super::schemas::AddTreePayload;
use super::schemas::FileUploadResponse;
use super::schemas::*;
use crate::services::AppState;
use crate::types::AddTreeRequest;
use crate::types::AddedTreesRequest;
use crate::types::CommentList;
use crate::types::GetTreesRequest;
use crate::types::MoveTreeRequest;
use crate::types::NewTreeDefaultsResponse;
use crate::types::PropList;
use crate::types::ReplaceTreeRequest;
use crate::types::SingleTreeResponse;
use crate::types::TreeList;
use crate::types::TreeStatsResponse;
use crate::types::UpdateTreeRequest;
use crate::types::UpdateTreeThumbnailRequest;
use crate::types::{AddCommentRequest, Error, Result};
use crate::utils::{get_remote_addr, get_user_agent};
use actix_web::web::{Bytes, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest, HttpResponse};
use log::debug;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PathInfo {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
struct RequestPayload {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePhotos {
    pub files: Vec<String>,
}

#[post("/{id}/comments")]
pub async fn add_comment_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<RequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .add_comment_handler
        .handle(AddCommentRequest {
            tree_id: path.id,
            message: payload.message.to_string(),
            user_id,
        })
        .await?;

    Ok(HttpResponse::Accepted().finish())
}

#[post("/{id}/files")]
pub async fn add_file_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
    body: Bytes,
) -> Result<Json<FileUploadResponse>> {
    let user_id = state.get_user_id(&req)?;

    let file = state
        .files
        .add_file(AddFileRequest {
            user_id,
            tree_id: path.id,
            remote_addr: get_remote_addr(&req).ok_or(Error::RemoteAddrNotSet)?,
            user_agent: get_user_agent(&req).ok_or(Error::UserAgentNotSet)?,
            file: body.to_vec(),
        })
        .await?;

    Ok(Json(FileUploadResponse::from_file(&file)))
}

#[post("/{id}/photos")]
pub async fn add_photos_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdatePhotos>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    let req = AddPhotosRequest {
        tree_id: path.id,
        files: payload.files.clone(),
        user_id,
    };

    state.add_photos_handler.handle(req).await?;

    Ok(HttpResponse::Accepted().finish())
}

#[post("")]
pub async fn add_trees_action(
    state: Data<AppState>,
    payload: Json<AddTreePayload>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let user_id = state.get_user_id(&req)?;

    let trees = state
        .add_trees_handler
        .handle(AddTreeRequest {
            points: payload.points.clone(),
            species: payload.species.clone(),
            notes: payload.notes.clone(),
            height: payload.height,
            circumference: payload.circumference,
            diameter: payload.diameter,
            state: payload.state.clone(),
            user_id,
            year: payload.year,
            files: payload.files.clone(),
            address: payload.address.clone(),
        })
        .await?;

    Ok(Json(TreeList::from_trees(&trees)))
}

#[get("/new")]
pub async fn get_new_trees_action(
    state: Data<AppState>,
    query: Query<AddedTreesRequest>,
) -> Result<Json<TreeList>> {
    let count = query.get_count();
    let skip = query.get_skip();
    let trees = state.get_new_trees_handler.handle(count, skip).await?;
    Ok(Json(trees))
}

#[get("/{id}")]
pub async fn get_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<SingleTreeResponse>> {
    let tree = state.get_tree_handler.handle(path.id).await?;
    Ok(Json(tree))
}

#[get("{id}/comments")]
pub async fn get_tree_comments_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<CommentList>> {
    let comments = state.get_tree_comments_handler.handle(path.id).await?;
    Ok(Json(comments))
}

#[get("/defaults")]
pub async fn get_tree_defaults_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<NewTreeDefaultsResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = state.get_tree_defaults_handler.handle(user_id).await?;
    Ok(Json(response))
}

#[get("/{id}/history")]
pub async fn get_tree_history_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<PropList>> {
    let comments = state.get_tree_history_handler.handle(path.id).await?;
    Ok(Json(comments))
}

#[get("/stats")]
pub async fn get_tree_stats_action(state: Data<AppState>) -> Result<Json<TreeStatsResponse>> {
    let stats = state.get_tree_stats_handler.handle().await?;
    Ok(Json(stats))
}

#[get("")]
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

#[get("/updated")]
pub async fn get_updated_trees_action(
    state: Data<AppState>,
    query: Query<AddedTreesRequest>,
) -> Result<Json<TreeList>> {
    let count = query.get_count();
    let skip = query.get_skip();
    let trees = state.get_updated_trees_handler.handle(count, skip).await?;
    Ok(Json(trees))
}

#[post("/{id}/likes")]
pub async fn like_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;
    state.like_tree_handler.handle(path.id, user_id).await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}

#[put("/{id}/position")]
pub async fn move_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<MoveRequestPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    state
        .move_tree_handler
        .handle(MoveTreeRequest {
            id: path.id,
            lat: payload.lat,
            lon: payload.lon,
            user_id,
        })
        .await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}

#[put("/{id}/replace")]
pub async fn replace_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<ReplaceTreeRequestPayload>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let trees = state
        .replace_tree_handler
        .handle(ReplaceTreeRequest {
            id: path.id,
            user_id: state.get_user_id(&req)?,
            species: payload.species.clone(),
            notes: payload.notes.clone(),
            height: payload.height,
            circumference: payload.circumference,
            diameter: payload.diameter,
            state: payload.state.clone(),
            year: payload.year,
            files: payload.files.clone(),
        })
        .await?;

    Ok(Json(TreeList::from_trees(&trees)))
}

#[delete("/{id}/likes")]
pub async fn unlike_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;
    state.unlike_tree_handler.handle(path.id, user_id).await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}

#[put("/{id}")]
pub async fn update_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdateTreeRequestPayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;

    let tree = state
        .update_tree_handler
        .handle(UpdateTreeRequest {
            id: path.id,
            lat: payload.lat,
            lon: payload.lon,
            species: payload.species.clone(),
            notes: payload.notes.clone(),
            height: payload.height,
            circumference: payload.circumference,
            diameter: payload.diameter,
            state: payload.state.clone(),
            user_id,
            year: payload.year,
            address: payload.address.clone(),
        })
        .await?;

    Ok(Json(tree))
}

#[put("/{id}/circumference")]
pub async fn update_tree_circumference_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdateCircumferencePayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let tree = state
        .update_tree_circumference_handler
        .handle(path.id, payload.value, user_id)
        .await?;
    Ok(Json(tree))
}

#[put("/{id}/diameter")]
pub async fn update_tree_diameter_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdateDiameterPayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let tree = state
        .update_tree_diameter_handler
        .handle(path.id, payload.value, user_id)
        .await?;
    Ok(Json(tree))
}

#[put("/{id}/height")]
pub async fn update_tree_height_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdateHeightPayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let tree = state
        .update_tree_height_handler
        .handle(path.id, payload.value, user_id)
        .await?;
    Ok(Json(tree))
}

#[put("/{id}/location")]
pub async fn update_tree_location_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdateLocationPayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;
    let tree = state
        .update_tree_location_handler
        .handle(path.id, payload.lat, payload.lon, user_id)
        .await?;
    Ok(Json(tree))
}

#[put("/{id}/state")]
pub async fn update_tree_state_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<UpdateStatePayload>,
    req: HttpRequest,
) -> Result<Json<SingleTreeResponse>> {
    let user_id = state.get_user_id(&req)?;

    let tree = state
        .update_tree_state_handler
        .handle(
            path.id,
            payload.value.clone(),
            user_id,
            payload.comment.clone(),
        )
        .await?;

    Ok(Json(tree))
}

#[put("/{id}/thumbnail")]
pub async fn update_tree_thumbnail_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<ThumbnailPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;

    let req = UpdateTreeThumbnailRequest {
        tree_id: path.id,
        file_id: payload.file.parse().map_err(|_| Error::BadImage)?,
        user_id,
    };

    state.update_tree_thumbnail_handler.handle(req).await?;

    Ok(HttpResponse::Accepted()
        .content_type("application/json")
        .finish())
}

// Configure the router.
pub fn tree_router(cfg: &mut ServiceConfig) {
    cfg.service(add_comment_action)
        .service(add_file_action)
        .service(add_photos_action)
        .service(add_trees_action)
        .service(get_new_trees_action)
        .service(get_tree_action)
        .service(get_tree_comments_action)
        .service(get_tree_defaults_action)
        .service(get_tree_history_action)
        .service(get_tree_stats_action)
        .service(get_trees_action)
        .service(get_updated_trees_action)
        .service(like_tree_action)
        .service(move_tree_action)
        .service(replace_tree_action)
        .service(unlike_tree_action)
        .service(update_tree_action)
        .service(update_tree_circumference_action)
        .service(update_tree_diameter_action)
        .service(update_tree_height_action)
        .service(update_tree_location_action)
        .service(update_tree_state_action)
        .service(update_tree_thumbnail_action);
}
