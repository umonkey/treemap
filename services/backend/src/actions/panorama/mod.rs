use actix_web::web::ServiceConfig;

pub use actions::*;
mod actions;
mod schemas;

pub fn panorama_router(cfg: &mut ServiceConfig) {
    cfg.service(list_panoramas_action)
        .service(create_panorama_action)
        .service(get_panorama_action)
        .service(update_panorama_action)
        .service(verify_video_upload_action)
        .service(start_video_multipart_action)
        .service(complete_video_multipart_action);
}
