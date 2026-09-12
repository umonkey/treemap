use crate::domain::alert::Alert;
use crate::domain::panorama::{group_visible, Panorama, PanoramaImage};
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
    group_visible(images, |t| t.0.panorama_id, |t| t.0.hidden)
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
        let raw_points = match &pan.points_json {
            Some(raw) if !raw.is_empty() => raw,
            _ => continue,
        };

        let coordinates: Value = match serde_json::from_str(raw_points) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let Value::Array(sections) = coordinates else {
            continue;
        };

        if sections.is_empty() {
            continue;
        }

        if !matches!(sections.first(), Some(Value::Array(_))) {
            continue;
        }

        let legacy = matches!(
            sections.first(),
            Some(Value::Array(first)) if first.first().is_some_and(Value::is_number)
        );

        // Depth-2 coordinates are a flat LineString: wrap them into one section.
        let normalized = if legacy {
            Value::Array(vec![Value::Array(sections)])
        } else {
            Value::Array(sections)
        };

        features.push(json!({
            "type": "Feature",
            "id": pan.id.to_string(),
            "geometry": {
                "type": "MultiLineString",
                "coordinates": normalized
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

    fn panorama(id: u64, points_json: Option<&str>) -> Panorama {
        Panorama {
            id,
            storage_key: id.to_string(),
            created_at: 0,
            created_by: 0,
            image_count: 2,
            file_size: None,
            processing_time: None,
            status: crate::domain::panorama::PanoramaStatus::Success,
            title: "test".to_string(),
            visible: true,
            source_video_path: None,
            gpx_path: None,
            video_timestamp: None,
            lat_offset: 0.0,
            lon_offset: 0.0,
            processing_arn: None,
            processing_status: None,
            failure_reason: None,
            min_lat: None,
            max_lat: None,
            min_lon: None,
            max_lon: None,
            points_json: points_json.map(str::to_string),
        }
    }

    #[tokio::test]
    async fn respond_with_panoramas_emits_multi_line_string_sections() {
        let panorama = panorama(
            7,
            Some("[[[44.0,40.0],[44.1,40.1]],[[44.2,40.2],[44.3,40.3]]]"),
        );

        let response = respond_with_panoramas(&[], &[panorama]);

        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .unwrap();
        let value: Value = serde_json::from_slice(&body).unwrap();
        let feature = &value["features"][0];

        assert_eq!(feature["geometry"]["type"], "MultiLineString");
        assert_eq!(
            feature["geometry"]["coordinates"],
            json!([[[44.0, 40.0], [44.1, 40.1]], [[44.2, 40.2], [44.3, 40.3]]])
        );
    }

    #[tokio::test]
    async fn respond_with_panoramas_normalizes_legacy_flat_points() {
        let panorama = panorama(8, Some("[[44.0,40.0],[44.1,40.1]]"));

        let response = respond_with_panoramas(&[], &[panorama]);

        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .unwrap();
        let value: Value = serde_json::from_slice(&body).unwrap();
        let feature = &value["features"][0];

        assert_eq!(feature["geometry"]["type"], "MultiLineString");
        assert_eq!(
            feature["geometry"]["coordinates"],
            json!([[[44.0, 40.0], [44.1, 40.1]]])
        );
    }

    #[tokio::test]
    async fn respond_with_panoramas_skips_panoramas_without_points() {
        let panorama = panorama(9, None);

        let response = respond_with_panoramas(&[], &[panorama]);

        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .unwrap();
        let value: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(value["features"].as_array().unwrap().len(), 0);
    }
}
