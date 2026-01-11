use super::schemas::AddFileRequest;
use super::schemas::AddTreePayload;
use super::schemas::FileUploadResponse;
use super::schemas::*;
use crate::domain::tree::TreeStats;
use crate::services::comment_loader::CommentList;
use crate::services::tree_loader::SingleTreeResponse;
use crate::services::tree_loader::TreeList;
use crate::services::AppState;
use crate::types::AddTreeRequest;
use crate::types::AddedTreesRequest;
use crate::types::GetTreesRequest;
use crate::types::NewTreeDefaultsResponse;
use crate::types::PropList;
use crate::types::ReplaceTreeRequest;
use crate::types::UpdateTreeRequest;
use crate::types::{Error, Result};
use crate::utils::{get_remote_addr, get_user_agent};
use actix_web::web::{Bytes, Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpRequest, HttpResponse};
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
        .comments
        .add_comment(path.id, user_id, &payload.message)
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

    state
        .trees
        .add_tree_photos(path.id, user_id, payload.files.clone())
        .await?;

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
        .trees
        .add_trees(AddTreeRequest {
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
    let trees = state.trees.get_new_trees(count, skip).await?;
    let trees = state.tree_loader.load_list(&trees).await?;
    Ok(Json(trees))
}

#[get("/{id}")]
pub async fn get_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<SingleTreeResponse>> {
    let tree = state.trees.get_tree(path.id).await?;
    let res = state.tree_loader.load_single(&tree).await?;
    Ok(Json(res))
}

#[get("{id}/comments")]
pub async fn get_tree_comments_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<CommentList>> {
    let comments = state.comments.get_tree_comments(path.id).await?;
    let res = state.comment_loader.load(&comments).await?;
    Ok(Json(res))
}

#[get("/defaults")]
pub async fn get_tree_defaults_action(
    state: Data<AppState>,
    req: HttpRequest,
) -> Result<Json<NewTreeDefaultsResponse>> {
    let user_id = state.get_user_id(&req)?;
    let response = state.trees.get_defaults(user_id).await?;
    Ok(Json(response))
}

#[get("/{id}/history")]
pub async fn get_tree_history_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
) -> Result<Json<PropList>> {
    let comments = state.trees.get_history(path.id).await?;
    Ok(Json(comments))
}

#[get("/stats")]
pub async fn get_tree_stats_action(state: Data<AppState>) -> Result<Json<TreeStats>> {
    let stats = state.trees.get_stats().await?;
    Ok(Json(stats))
}

#[get("")]
pub async fn get_trees_action(
    state: Data<AppState>,
    query: Query<GetTreesRequest>,
    req: HttpRequest,
) -> Result<Json<TreeList>> {
    let user_id = state.get_user_id(&req).unwrap_or(0);
    let trees = state.trees.get_trees(&query, user_id).await?;

    let res = state.tree_loader.load_list(&trees).await?;

    Ok(Json(res))
}

#[get("/updated")]
pub async fn get_updated_trees_action(
    state: Data<AppState>,
    query: Query<AddedTreesRequest>,
) -> Result<Json<TreeList>> {
    let count = query.get_count();
    let skip = query.get_skip();
    let trees = state.trees.get_recently_updated(count, skip).await?;
    let res = state.tree_loader.load_list(&trees).await?;
    Ok(Json(res))
}

#[post("/{id}/likes")]
pub async fn like_tree_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;
    state.likes.like_tree(path.id, user_id).await?;

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
        .trees
        .move_tree(path.id, user_id, payload.lat, payload.lon)
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
        .trees
        .replace_tree(ReplaceTreeRequest {
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
    state.likes.unlike_tree(path.id, user_id).await?;

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
        .trees
        .update_tree(UpdateTreeRequest {
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

    let res = state.tree_loader.load_single(&tree).await?;

    Ok(Json(res))
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
        .trees
        .update_circumference(path.id, payload.value, user_id)
        .await?;

    let res = state.tree_loader.load_single(&tree).await?;

    Ok(Json(res))
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
        .trees
        .update_diameter(path.id, payload.value, user_id)
        .await?;

    let res = state.tree_loader.load_single(&tree).await?;

    Ok(Json(res))
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
        .trees
        .update_height(path.id, payload.value, user_id)
        .await?;

    let res = state.tree_loader.load_single(&tree).await?;

    Ok(Json(res))
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
        .trees
        .update_location(path.id, payload.lat, payload.lon, user_id)
        .await?;

    let res = state.tree_loader.load_single(&tree).await?;

    Ok(Json(res))
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
        .trees
        .update_state(
            path.id,
            payload.value.clone(),
            user_id,
            payload.comment.clone(),
        )
        .await?;

    let res = state.tree_loader.load_single(&tree).await?;

    Ok(Json(res))
}

#[put("/{id}/thumbnail")]
pub async fn update_tree_thumbnail_action(
    state: Data<AppState>,
    path: Path<PathInfo>,
    payload: Json<ThumbnailPayload>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = state.get_user_id(&req)?;
    let file_id = payload.file.parse().map_err(|_| Error::BadImage)?;

    state
        .trees
        .update_thumbnail(path.id, file_id, user_id)
        .await?;

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
