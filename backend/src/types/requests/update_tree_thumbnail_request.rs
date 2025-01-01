#[derive(Debug)]
pub struct UpdateTreeThumbnailRequest {
    pub tree_id: u64,
    pub file_id: u64,
    pub user_id: u64,
}
