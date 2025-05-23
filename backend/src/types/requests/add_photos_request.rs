//! This represents the request to add photos to a tree.
//! files are ids of the files previously uploaded via v1/upload.

#[derive(Default)]
pub struct AddPhotosRequest {
    pub tree_id: u64,
    pub files: Vec<String>,
    pub user_id: u64,
}
