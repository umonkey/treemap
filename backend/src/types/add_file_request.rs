pub struct AddFileRequest {
    pub user_id: u64,
    pub tree_id: u64,
    pub file: Vec<u8>,
}
