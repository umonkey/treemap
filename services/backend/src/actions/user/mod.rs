//! This module groups web actions related to user management.

mod actions;
mod schemas;

pub use actions::*;
pub use schemas::UserList;
pub use schemas::UserRead;

use actix_web::web::ServiceConfig;

pub fn user_router(cfg: &mut ServiceConfig) {
    cfg.service(get_users)
        .service(get_user_heatmap)
        .service(get_user)
        .service(update_user_action);
}
