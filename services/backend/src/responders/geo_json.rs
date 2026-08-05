use crate::domain::alert::Alert;
use crate::domain::panorama::{Panorama, PanoramaImage};
use crate::domain::tree::Tree;
use crate::utils::get_timestamp;
use actix_web::HttpResponse;
use serde_json::{json, Value};
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

pub fn respond_with_panoramas(
    images: &[(PanoramaImage, i64, f64, f64)],
    panoramas: &[Panorama],
) -> HttpResponse {
    let mut features = Vec::new();

    for (img, created_at, lat_offset, lon_offset) in images {
        let lat = img.lat + lat_offset;
        let lng = img.lng + lon_offset;
        features.push(json!({
            "type": "Feature",
            "id": img.id.to_string(),
            "geometry": {
                "type": "Point",
                "coordinates": [lng, lat]
            },
            "properties": {
                "id": img.id.to_string(),
                "sequence_id": img.panorama_id.to_string(),
                "captured_at": *created_at,
                "compass_angle": img.heading,
                "kind": "image"
            }
        }));
    }

    for pan in panoramas {
        let coords: Value =
            serde_json::from_str(&pan.points_json.clone().unwrap_or_default()).unwrap_or(json!([]));
        let adjusted_coords = if let Value::Array(arr) = coords {
            Value::Array(
                arr.into_iter()
                    .map(|pt| {
                        if let Value::Array(pt_arr) = pt {
                            if pt_arr.len() >= 2 {
                                let lon = pt_arr[0].as_f64().unwrap_or(0.0) + pan.lon_offset;
                                let lat = pt_arr[1].as_f64().unwrap_or(0.0) + pan.lat_offset;
                                json!([lon, lat])
                            } else {
                                Value::Array(pt_arr)
                            }
                        } else {
                            pt
                        }
                    })
                    .collect(),
            )
        } else {
            coords
        };
        features.push(json!({
            "type": "Feature",
            "id": pan.id.to_string(),
            "geometry": {
                "type": "LineString",
                "coordinates": adjusted_coords
            },
            "properties": {
                "id": pan.id.to_string(),
                "captured_at": pan.created_at,
                "image_count": pan.image_count,
                "kind": "sequence"
            }
        }));
    }

    let collection = json!({
        "type": "FeatureCollection",
        "features": features
    });

    HttpResponse::Ok()
        .content_type("application/geo+json")
        .json(collection)
}
