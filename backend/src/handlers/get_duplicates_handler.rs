use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::osm_round_coord;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GetDuplicatesHandler {
    trees: Arc<TreeRepository>,
}

impl GetDuplicatesHandler {
    pub async fn handle(&self) -> Result<DuplicatesResponse> {
        // Get all trees from the database
        let trees = self.trees.all().await?;

        // HashMap to store coordinate -> tree_ids mapping
        let mut location_map: HashMap<String, Vec<u64>> = HashMap::new();

        // Process each tree
        for tree in trees {
            // Round coordinates using OSM standard (7 decimal places)
            let rounded_lat = osm_round_coord(tree.lat);
            let rounded_lon = osm_round_coord(tree.lon);

            // Create location key
            let location_key = format!("{rounded_lat},{rounded_lon}");

            // Add tree ID to the location
            location_map.entry(location_key).or_default().push(tree.id);
        }

        // Collect locations with more than 1 tree
        let mut duplicates = Vec::new();
        for (location_key, tree_ids) in location_map {
            if tree_ids.len() > 1 {
                // Parse coordinates back from the key
                let coords: Vec<&str> = location_key.split(',').collect();
                if coords.len() == 2 {
                    if let (Ok(lat), Ok(lon)) = (coords[0].parse::<f64>(), coords[1].parse::<f64>())
                    {
                        duplicates.push(DuplicateLocation::new(lat, lon, tree_ids));
                    }
                }
            }
        }

        Ok(DuplicatesResponse::new(duplicates))
    }
}

impl Locatable for GetDuplicatesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_coordinate_rounding() {
        // Test the coordinate rounding function (OSM standard 7 decimal places)
        assert_eq!(osm_round_coord(40.1813891), 40.1813891);
        assert_eq!(osm_round_coord(40.18138999), 40.1813900);
        assert_eq!(osm_round_coord(-123.4194001), -123.4194001);
        assert_eq!(osm_round_coord(-123.41940009), -123.4194001);
    }

    #[test]
    fn test_duplicate_detection_logic() {
        // Test tree coordinates that should be detected as duplicates
        let test_trees = vec![
            (1, 40.1813891, 44.5144444),   // Tree 1
            (2, 40.1813891, 44.5144444),   // Tree 2 (exact duplicate)
            (3, 40.1813899, 44.5144449),   // Tree 3 (close, different at 7th decimal)
            (4, 38.7749000, -123.4194000), // Tree 4
            (5, 38.7749000, -123.4194000), // Tree 5 (exact duplicate)
            (6, 38.7749009, -123.4194009), // Tree 6 (close, different at 7th decimal)
            (7, 39.7749000, -124.4194000), // Tree 7 (unique)
        ];

        let mut location_map: HashMap<String, Vec<u64>> = HashMap::new();

        for (id, lat, lon) in test_trees {
            let rounded_lat = osm_round_coord(lat);
            let rounded_lon = osm_round_coord(lon);
            let location_key = format!("{rounded_lat},{rounded_lon}");
            location_map.entry(location_key).or_default().push(id);
        }

        // Collect duplicates
        let mut duplicates = Vec::new();
        for (location_key, tree_ids) in location_map {
            if tree_ids.len() > 1 {
                let coords: Vec<&str> = location_key.split(',').collect();
                let lat = coords[0].parse::<f64>().unwrap();
                let lon = coords[1].parse::<f64>().unwrap();
                duplicates.push((lat, lon, tree_ids));
            }
        }

        // Should have exactly 2 duplicate locations
        assert_eq!(duplicates.len(), 2);

        // Find the duplicate at (40.1813891, 44.5144444)
        let duplicate1 = duplicates.iter().find(|(lat, lon, _)| {
            (*lat - 40.1813891).abs() < 0.0000001 && (*lon - 44.5144444).abs() < 0.0000001
        });
        assert!(duplicate1.is_some());
        let (_, _, tree_ids1) = duplicate1.unwrap();
        assert_eq!(tree_ids1.len(), 2);
        assert!(tree_ids1.contains(&1));
        assert!(tree_ids1.contains(&2));

        // Find the duplicate at (38.7749000, -123.4194000)
        let duplicate2 = duplicates.iter().find(|(lat, lon, _)| {
            (*lat - 38.7749000).abs() < 0.0000001 && (*lon + 123.4194000).abs() < 0.0000001
        });
        assert!(duplicate2.is_some());
        let (_, _, tree_ids2) = duplicate2.unwrap();
        assert_eq!(tree_ids2.len(), 2);
        assert!(tree_ids2.contains(&4));
        assert!(tree_ids2.contains(&5));
    }
}
