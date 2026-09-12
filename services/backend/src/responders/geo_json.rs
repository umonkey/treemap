use crate::domain::alert::Alert;
use crate::domain::panorama::{Panorama, PanoramaImage};
use crate::domain::tree::Tree;
use crate::domain::water::WaterSource;
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

/// Convert a list of water sources to a GeoJSON FeatureCollection response.
pub fn respond_with_water(sources: &[WaterSource]) -> HttpResponse {
    let features: Vec<_> = sources
        .iter()
        .map(|source| {
            json!({
                "type": "Feature",
                "id": source.id.to_string(),
                "geometry": {
                    "type": "Point",
                    "coordinates": [source.lon, source.lat]
                },
                "properties": {
                    "id": source.id.to_string(),
                    "status": source.status,
                    "created_at": source.created_at,
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

/// Split images into runs of visible images.
///
/// Hidden images act as dividers and are never included in the result. A run is
/// also flushed when the `panorama_id` changes. Empty runs from consecutive
/// hidden images are discarded.
fn get_visible_groups(
    images: &[(PanoramaImage, i64, f64, f64)],
) -> Vec<Vec<&(PanoramaImage, i64, f64, f64)>> {
    let mut groups = Vec::new();
    let mut current: Vec<&(PanoramaImage, i64, f64, f64)> = Vec::new();
    let mut current_panorama_id: Option<u64> = None;

    for image in images {
        let panorama_id = image.0.panorama_id;
        let changed = current_panorama_id.is_some_and(|id| id != panorama_id);

        if (image.0.hidden || changed) && !current.is_empty() {
            groups.push(std::mem::take(&mut current));
        }

        current_panorama_id = Some(panorama_id);

        if !image.0.hidden {
            current.push(image);
        }
    }

    if !current.is_empty() {
        groups.push(current);
    }

    groups
}

/// Convert a single panorama's images to a GeoJSON FeatureCollection response.
///
/// Emits one Point feature per image (hidden images included) followed by a
/// LineString feature for each run of visible images with at least two points.
pub fn respond_with_panorama(images: &[(PanoramaImage, i64, f64, f64)]) -> HttpResponse {
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
                "kind": "image",
                "hidden": img.hidden,
            }
        }));
    }

    for group in get_visible_groups(images) {
        if group.len() < 2 {
            continue;
        }

        let coordinates: Vec<Value> = group
            .iter()
            .map(|(img, _, lat_offset, lon_offset)| {
                json!([img.lng + lon_offset, img.lat + lat_offset])
            })
            .collect();

        let panorama_id = group[0].0.panorama_id.to_string();

        features.push(json!({
            "type": "Feature",
            "id": panorama_id,
            "geometry": {
                "type": "LineString",
                "coordinates": coordinates
            },
            "properties": {
                "id": panorama_id,
                "captured_at": group[0].1,
                "image_count": group.len(),
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
                "kind": "image",
                "hidden": img.hidden,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn image(id: u64, panorama_id: u64, hidden: bool) -> (PanoramaImage, i64, f64, f64) {
        (
            PanoramaImage {
                id,
                panorama_id,
                filename: format!("{id}.jpg"),
                lat: 0.0,
                lng: 0.0,
                heading: 0.0,
                pitch: 0.0,
                roll: 0.0,
                hidden,
            },
            0,
            0.0,
            0.0,
        )
    }

    fn ids(groups: &[Vec<&(PanoramaImage, i64, f64, f64)>]) -> Vec<Vec<u64>> {
        groups
            .iter()
            .map(|group| group.iter().map(|(img, ..)| img.id).collect())
            .collect()
    }

    #[test]
    fn no_hidden_images_yields_one_group() {
        let images = vec![image(1, 1, false), image(2, 1, false), image(3, 1, false)];

        let groups = get_visible_groups(&images);

        assert_eq!(ids(&groups), vec![vec![1, 2, 3]]);
    }

    #[test]
    fn hidden_image_splits_into_two_groups() {
        let images = vec![image(1, 1, false), image(2, 1, true), image(3, 1, false)];

        let groups = get_visible_groups(&images);

        assert_eq!(ids(&groups), vec![vec![1], vec![3]]);
    }

    #[test]
    fn consecutive_hidden_images_produce_no_empty_groups() {
        let images = vec![
            image(1, 1, false),
            image(2, 1, true),
            image(3, 1, true),
            image(4, 1, false),
        ];

        let groups = get_visible_groups(&images);

        assert_eq!(ids(&groups), vec![vec![1], vec![4]]);
    }

    #[test]
    fn singleton_visible_between_hidden_images_is_returned() {
        let images = vec![image(1, 1, true), image(2, 1, false), image(3, 1, true)];

        let groups = get_visible_groups(&images);

        assert_eq!(ids(&groups), vec![vec![2]]);
    }

    #[test]
    fn panorama_id_change_splits_without_hidden_image() {
        let images = vec![image(1, 1, false), image(2, 2, false)];

        let groups = get_visible_groups(&images);

        assert_eq!(ids(&groups), vec![vec![1], vec![2]]);
    }
}
