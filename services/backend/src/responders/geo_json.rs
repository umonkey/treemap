use crate::domain::alert::Alert;
use crate::domain::tree::Tree;
use crate::utils::get_timestamp;
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

/// Convert a list of alerts to a GeoJSON FeatureCollection response.
pub fn respond_with_alerts(alerts: &[Alert], days: u64) -> HttpResponse {
    let now = get_timestamp();
    let max_age = (days * 24 * 60 * 60) as f64;

    let features: Vec<_> = alerts
        .iter()
        .map(|alert| {
            let age = now.saturating_sub(alert.created_at) as f64;
            let weight = (1.0 - (age / max_age)).clamp(0.0, 1.0);

            json!({
                "type": "Feature",
                "id": alert.id.to_string(),
                "geometry": {
                    "type": "Point",
                    "coordinates": [alert.lon.unwrap_or(0.0), alert.lat.unwrap_or(0.0)]
                },
                "properties": {
                    "id": alert.id.to_string(),
                    "created_at": alert.created_at,
                    "description": alert.description,
                    "status": alert.status,
                    "weight": weight,
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
