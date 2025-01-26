#[derive(Default)]
pub struct AddFileRequest {
    pub user_id: u64,
    pub tree_id: u64,
    pub file: Vec<u8>,
    pub remote_addr: String,
    pub user_agent: String,
}
