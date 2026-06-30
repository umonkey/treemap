mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn comment_router(cfg: &mut ServiceConfig) {
    cfg.service(get_new_comments_action);
}
