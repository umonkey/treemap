mod actions;
pub mod schemas;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn upload_router(cfg: &mut ServiceConfig) {
    cfg.service(upload_action).service(finish_upload_action);
}
