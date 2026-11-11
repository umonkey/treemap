mod actions;

pub use actions::*;

use actix_web::web::ServiceConfig;

pub fn comment_router(cfg: &mut ServiceConfig) {
    cfg.service(add_comment_action)
        .service(get_comments_action)
        .service(get_new_comments_action);
}
