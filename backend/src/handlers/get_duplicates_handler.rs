use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::round_coord_6_digits;
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
            // Round coordinates to 6 decimal places
            let rounded_lat = round_coord_6_digits(tree.lat);
            let rounded_lon = round_coord_6_digits(tree.lon);
            
            // Create location key
            let location_key = format!("{},{}", rounded_lat, rounded_lon);
            
            // Add tree ID to the location
            location_map.entry(location_key).or_insert_with(Vec::new).push(tree.id);
        }
        
        // Collect locations with more than 1 tree
        let mut duplicates = Vec::new();
        for (location_key, tree_ids) in location_map {
            if tree_ids.len() > 1 {
                // Parse coordinates back from the key
                let coords: Vec<&str> = location_key.split(',').collect();
                if coords.len() == 2 {
                    if let (Ok(lat), Ok(lon)) = (coords[0].parse::<f64>(), coords[1].parse::<f64>()) {
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
        // Test the coordinate rounding function
        assert_eq!(round_coord_6_digits(40.1813891), 40.181389);
        assert_eq!(round_coord_6_digits(40.1813899), 40.18139);
        assert_eq!(round_coord_6_digits(-123.4194001), -123.419400);
        assert_eq!(round_coord_6_digits(-123.4194009), -123.419401);
    }

    #[test]
    fn test_duplicate_detection_logic() {
        // Test tree coordinates that should be detected as duplicates
        let test_trees = vec![
            (1, 40.181389, 44.514444),   // Tree 1
            (2, 40.181389, 44.514444),   // Tree 2 (exact duplicate)
            (3, 40.181390, 44.514445),   // Tree 3 (close, different)
            (4, 38.7749, -123.4194),     // Tree 4
            (5, 38.7749, -123.4194),     // Tree 5 (exact duplicate)
            (6, 38.774901, -123.419401), // Tree 6 (close, different)
            (7, 39.7749, -124.4194),     // Tree 7 (unique)
        ];

        let mut location_map: HashMap<String, Vec<u64>> = HashMap::new();

        for (id, lat, lon) in test_trees {
            let rounded_lat = round_coord_6_digits(lat);
            let rounded_lon = round_coord_6_digits(lon);
            let location_key = format!("{},{}", rounded_lat, rounded_lon);
            location_map.entry(location_key).or_insert_with(Vec::new).push(id);
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

        // Find the duplicate at (40.181389, 44.514444)
        let duplicate1 = duplicates.iter().find(|(lat, lon, _)| {
            (*lat - 40.181389).abs() < 0.000001 && (*lon - 44.514444).abs() < 0.000001
        });
        assert!(duplicate1.is_some());
        let (_, _, tree_ids1) = duplicate1.unwrap();
        assert_eq!(tree_ids1.len(), 2);
        assert!(tree_ids1.contains(&1));
        assert!(tree_ids1.contains(&2));

        // Find the duplicate at (38.7749, -123.4194)
        let duplicate2 = duplicates.iter().find(|(lat, lon, _)| {
            (*lat - 38.7749).abs() < 0.000001 && (*lon + 123.4194).abs() < 0.000001
        });
        assert!(duplicate2.is_some());
        let (_, _, tree_ids2) = duplicate2.unwrap();
        assert_eq!(tree_ids2.len(), 2);
        assert!(tree_ids2.contains(&4));
        assert!(tree_ids2.contains(&5));
    }
}