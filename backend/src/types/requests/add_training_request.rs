use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AddTrainingRequest {
    pub user_id: u64,
    pub result: f64,
}
