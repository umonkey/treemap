mod models;
mod repository;
mod schemas;
mod service;

pub use models::WaterSource;
pub use models::WaterStatus;
pub use repository::WaterRepository;
pub use schemas::AddWaterRequest;
pub use schemas::GetWaterRequest;
pub use schemas::UpdateWaterRequest;
pub use service::WaterService;
