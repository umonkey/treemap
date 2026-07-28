use actix_web::web::ServiceConfig;

pub use actions::*;
pub use schemas::*;
mod actions;
mod schemas;

pub fn panorama_router(cfg: &mut ServiceConfig) {
    cfg.service(list_panoramas_action)
        .service(create_panorama_action)
        .service(get_panoramas_geo_json_action)
        .service(get_panorama_hints_action)
        .service(get_panorama_action)
        .service(update_panorama_action)
        .service(verify_video_upload_action)
        .service(start_video_multipart_action)
        .service(complete_video_multipart_action)
        .service(get_track_upload_url_action)
        .service(verify_track_upload_action)
        .service(get_web_video_url_action)
        .service(get_panorama_track_action)
        .service(get_panorama_image_action)
        .service(get_panorama_image_hints_action)
        .service(add_panorama_image_hint_action)
        .service(delete_panorama_image_hints_action);
}
