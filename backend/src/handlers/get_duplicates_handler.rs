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