use crate::utils::gpx::GpxPoint;
use actix_web::HttpResponse;
use serde_json::json;

pub fn respond_with_gpx(points: &[GpxPoint]) -> HttpResponse {
    let coordinates: Vec<[f64; 2]> = points.iter().map(|p| [p.lon, p.lat]).collect();
    let timestamps: Vec<Option<String>> = points.iter().map(|p| p.time.clone()).collect();

    let feature = json!({
        "type": "Feature",
        "geometry": {
            "type": "LineString",
            "coordinates": coordinates
        },
        "properties": {
            "timestamps": timestamps
        }
    });

    let collection = json!({
        "type": "FeatureCollection",
        "features": [feature]
    });

    HttpResponse::Ok()
        .content_type("application/geo+json")
        .json(collection)
}
