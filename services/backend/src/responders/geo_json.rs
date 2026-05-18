use crate::domain::report::Report;
use crate::domain::tree::Tree;
use actix_web::HttpResponse;
use serde_json::json;
use std::f64::consts::PI;

/// Convert a list of trees to a GeoJSON FeatureCollection response.
pub fn respond_with_trees(trees: &[Tree]) -> HttpResponse {
    let features: Vec<_> = trees
        .iter()
        .map(|tree| {
            let crown = if tree.is_existing() {
                tree.diameter.filter(|&d| d > 0.0).unwrap_or(4.0)
            } else {
                1.0
            };

            let trunk = tree
                .circumference
                .filter(|&c| c > 0.0)
                .map(|c| c / PI)
                .unwrap_or(0.0);

            json!({
                "type": "Feature",
                "id": tree.id.to_string(),
                "geometry": {
                    "type": "Point",
                    "coordinates": [tree.lon, tree.lat]
                },
                "properties": {
                    "id": tree.id.to_string(),
                    "crown": crown,
                    "trunk": trunk,
                    "state": tree.state,
                }
            })
        })
        .collect();

    let collection = json!({
        "type": "FeatureCollection",
        "features": features
    });

    HttpResponse::Ok()
        .content_type("application/geo+json")
        .json(collection)
}

/// Convert a list of reports to a GeoJSON FeatureCollection response.
pub fn respond_with_reports(reports: &[Report]) -> HttpResponse {
    let features: Vec<_> = reports
        .iter()
        .map(|report| {
            json!({
                "type": "Feature",
                "id": report.id.to_string(),
                "geometry": {
                    "type": "Point",
                    "coordinates": [report.lon.unwrap_or(0.0), report.lat.unwrap_or(0.0)]
                },
                "properties": {
                    "id": report.id.to_string(),
                    "created_at": report.created_at,
                    "description": report.description,
                    "status": report.status,
                }
            })
        })
        .collect();

    let collection = json!({
        "type": "FeatureCollection",
        "features": features
    });

    HttpResponse::Ok()
        .content_type("application/geo+json")
        .json(collection)
}
