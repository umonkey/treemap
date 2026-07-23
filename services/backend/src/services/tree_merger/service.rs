use crate::domain::comment::CommentRepository;
use crate::domain::like::LikeRepository;
use crate::domain::observation::ObservationRepository;
use crate::domain::osm::OsmTreeRepository;
use crate::domain::prop::{PropRecord, PropRepository};
use crate::domain::tree::{DuplicateLocation, TreeRepository};
use crate::domain::tree_image::TreeImageRepository;
use crate::services::{Context, Injectable};
use crate::types::*;
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;

const EXCLUDED_MERGE_PROPS: &[&str] = &["replaced_by", "replaces"];

pub struct TreeMergerService {
    trees: Arc<TreeRepository>,
    files: Arc<TreeImageRepository>,
    comments: Arc<CommentRepository>,
    likes: Arc<LikeRepository>,
    observations: Arc<ObservationRepository>,
    props: Arc<PropRepository>,
    osm_trees: Arc<OsmTreeRepository>,
    bot_user_id: u64,
}

impl TreeMergerService {
    pub async fn get_duplicates(&self) -> Result<Vec<DuplicateLocation>> {
        // Get all trees from the database
        let trees = self.trees.get_lightweight_locations().await?;

        // HashMap to store coordinate -> tree_ids mapping
        // Key is (lat * 10,000,000, lon * 10,000,000)
        let mut location_map: HashMap<(i64, i64), Vec<String>> = HashMap::new();

        // Process each tree
        for tree in trees {
            if tree.state == "replaced" {
                continue; // Skip trees that are not visible
            }

            // Round coordinates using OSM standard (7 decimal places)
            let rounded_lat = (tree.lat * 10_000_000.0).round() as i64;
            let rounded_lon = (tree.lon * 10_000_000.0).round() as i64;

            // Add tree ID to the location
            location_map
                .entry((rounded_lat, rounded_lon))
                .or_default()
                .push(tree.id.to_string());
        }

        // Collect locations with more than 1 tree
        let mut duplicates = Vec::new();

        for (coords, tree_ids) in location_map {
            if tree_ids.len() > 1 {
                let lat = coords.0 as f64 / 10_000_000.0;
                let lon = coords.1 as f64 / 10_000_000.0;
                duplicates.push(DuplicateLocation::new(lat, lon, tree_ids));
            }
        }

        debug!("Returning {} duplicate locations.", duplicates.len());

        Ok(duplicates)
    }

    pub async fn merge_duplicates(&self, limit: u64) -> Result<Vec<(u64, u64)>> {
        let duplicate_groups = self.get_duplicates().await?;
        let groups_to_process = duplicate_groups.into_iter().take(limit as usize);
        let mut merged_pairs = Vec::new();

        for group in groups_to_process {
            let tree_ids: Vec<u64> = group
                .tree_ids
                .iter()
                .filter_map(|id| id.parse::<u64>().ok())
                .collect();

            if tree_ids.len() < 2 {
                continue;
            }

            let mut trees = self.trees.get_multiple(&tree_ids).await?;

            if trees.is_empty() {
                continue;
            }

            // (3) Should pick the lowest id (first created)
            trees.sort_by_key(|t| t.id);
            let main_tree = trees[0].clone();
            let secondary_trees = &trees[1..];

            let mut merged_tree = main_tree.clone();

            // (4) Should merge all props, latest takes precedence
            // scalar props: species, height, circumference, diameter, notes, year, address

            // Collect all trees sorted by updated_at DESC to find latest values
            let mut latest_first = trees.clone();
            latest_first.sort_by_key(|b| std::cmp::Reverse(b.updated_at));

            // Merge species
            if let Some(t) = latest_first.iter().find(|t| {
                !t.species.is_empty()
                    && t.species != "Unknown"
                    && t.species != "Unknown species"
                    && !t.species.to_lowercase().contains("unknown")
            }) {
                merged_tree.species = t.species.clone();
            }

            // Merge state: ignore "gone"
            if let Some(t) = latest_first.iter().find(|t| t.state != "gone") {
                merged_tree.state = t.state.clone();
            }

            // Merge height
            if let Some(t) = trees
                .iter()
                .filter(|t| t.height.is_some() && t.height.unwrap_or(0.0) > 0.0)
                .max_by_key(|t| t.height_updated_at)
            {
                merged_tree.height = t.height;
                merged_tree.height_updated_at = t.height_updated_at;
            }

            // Merge circumference
            if let Some(t) = trees
                .iter()
                .filter(|t| t.circumference.is_some() && t.circumference.unwrap_or(0.0) > 0.0)
                .max_by_key(|t| t.circumference_updated_at)
            {
                merged_tree.circumference = t.circumference;
                merged_tree.circumference_updated_at = t.circumference_updated_at;
            }

            // Merge diameter
            if let Some(t) = trees
                .iter()
                .filter(|t| t.diameter.is_some() && t.diameter.unwrap_or(0.0) > 0.0)
                .max_by_key(|t| t.diameter_updated_at)
            {
                merged_tree.diameter = t.diameter;
                merged_tree.diameter_updated_at = t.diameter_updated_at;
            }

            // Merge notes
            if let Some(t) = latest_first
                .iter()
                .find(|t| t.notes.as_ref().map(|n| !n.is_empty()).unwrap_or(false))
            {
                merged_tree.notes = t.notes.clone();
            }

            // Merge year
            if let Some(t) = latest_first.iter().find(|t| t.year.is_some()) {
                merged_tree.year = t.year;
            }

            // Merge address
            if let Some(t) = latest_first
                .iter()
                .find(|t| t.address.as_ref().map(|a| !a.is_empty()).unwrap_or(false))
            {
                merged_tree.address = t.address.clone();
            }

            // Merge metadata updated_at
            merged_tree.images_updated_at =
                trees.iter().map(|t| t.images_updated_at).max().unwrap_or(0);
            merged_tree.observations_updated_at = trees
                .iter()
                .map(|t| t.observations_updated_at)
                .max()
                .unwrap_or(0);

            // Merge thumbnail_id: if main has none, take latest available
            if merged_tree.thumbnail_id.is_none() {
                if let Some(t) = trees
                    .iter()
                    .filter(|t| t.thumbnail_id.is_some())
                    .max_by_key(|t| t.images_updated_at)
                {
                    merged_tree.thumbnail_id = t.thumbnail_id;
                }
            }

            // Update main tree with merged values
            self.trees.update(&merged_tree, self.bot_user_id).await?;

            for secondary in secondary_trees {
                // (5) Should move all photos into the main tree
                self.files.reassign_all(secondary.id, main_tree.id).await?;

                // Move comments
                self.comments
                    .reassign_all(secondary.id, main_tree.id)
                    .await?;

                // Move likes
                self.likes.reassign_all(secondary.id, main_tree.id).await?;

                // Move observations
                self.observations
                    .reassign_all(secondary.id, main_tree.id)
                    .await?;

                // Move props history
                let secondary_props = self.props.find_by_tree(secondary.id).await?;

                let prop_ids_to_move: Vec<u64> = secondary_props
                    .into_iter()
                    .filter(|p| {
                        if EXCLUDED_MERGE_PROPS.contains(&p.name.as_str()) {
                            return false;
                        }

                        if p.name == "state" && p.value == "replaced" {
                            return false;
                        }

                        true
                    })
                    .map(|p| p.id)
                    .collect();

                self.props
                    .update_tree_id(prop_ids_to_move, main_tree.id)
                    .await?;

                // Add audit trail entry
                self.props
                    .add(&PropRecord {
                        tree_id: main_tree.id,
                        name: "merged_from".to_string(),
                        value: secondary.id.to_string(),
                        added_by: self.bot_user_id,
                        ..Default::default()
                    })
                    .await?;

                // (6) Should mark merged trees as replaced, with replaced_by pointing to the new tree.
                self.trees
                    .mark_as_merged(secondary.id, main_tree.id, self.bot_user_id)
                    .await?;

                info!("Tree {} merged into {}.", secondary.id, main_tree.id);

                merged_pairs.push((secondary.id, main_tree.id));
            }

            // Recalculate stats for the main tree
            self.trees.recalculate_stats(main_tree.id).await?;
        }

        Ok(merged_pairs)
    }

    pub async fn remap_osm_duplicates(&self) -> Result<Vec<(u64, u64)>> {
        let mismatched = self.trees.get_mismatched_osm_trees().await?;
        let mut remapped = Vec::new();

        for main_tree in mismatched {
            let main_osm_id = match main_tree.osm_id {
                Some(id) => id,
                None => continue,
            };

            let duplicates = self.trees.get_by_replaced_by(main_tree.id).await?;

            for dup_tree in duplicates {
                let dup_osm_id = match dup_tree.osm_id {
                    Some(id) => id,
                    None => continue,
                };

                // Check if the duplicate's OSM node is visible
                if let Ok(Some(osm_node)) = self.osm_trees.get(dup_osm_id).await {
                    if osm_node.visible {
                        info!(
                            "Swapping OSM IDs: main tree {} (OSM {}) <-> duplicate tree {} (OSM {})",
                            main_tree.id, main_osm_id, dup_tree.id, dup_osm_id
                        );

                        // Swap OSM IDs avoiding unique constraint violation
                        let mut updated_main = main_tree.clone();
                        let mut updated_dup = dup_tree.clone();

                        // 1. Clear duplicate's OSM ID
                        updated_dup.osm_id = None;
                        self.trees.update(&updated_dup, self.bot_user_id).await?;

                        // 2. Set main's OSM ID to the new one
                        updated_main.osm_id = Some(dup_osm_id);
                        self.trees.update(&updated_main, self.bot_user_id).await?;

                        // 3. Set duplicate's OSM ID to the old main one
                        updated_dup.osm_id = Some(main_osm_id);
                        self.trees.update(&updated_dup, self.bot_user_id).await?;

                        remapped.push((main_tree.id, dup_tree.id));
                        break; // Only one swap per main tree
                    }
                }
            }
        }

        Ok(remapped)
    }
}

impl Injectable for TreeMergerService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            files: Arc::new(ctx.build::<TreeImageRepository>()?),
            comments: Arc::new(ctx.build::<CommentRepository>()?),
            likes: Arc::new(ctx.build::<LikeRepository>()?),
            observations: Arc::new(ctx.build::<ObservationRepository>()?),
            props: Arc::new(ctx.build::<PropRepository>()?),
            osm_trees: Arc::new(ctx.build::<OsmTreeRepository>()?),
            bot_user_id: ctx.config().bot_user_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tree::Tree;
    use crate::infra::database::Database;
    use crate::services::AppState;
    use crate::services::ContextExt;
    use crate::utils::{get_timestamp, osm_round_coord};

    async fn setup() -> (Arc<TreeMergerService>, Arc<Database>) {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        let merger = Arc::new(
            state
                .build::<TreeMergerService>()
                .expect("Error creating TreeMergerService"),
        );

        (merger, state.database())
    }

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
                let lat = coords[0].parse::<f64>().expect("Error reading lat");
                let lon = coords[1].parse::<f64>().expect("Error reading lon");
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
        let (_, _, tree_ids1) = duplicate1.expect("Error getting duplicates.");
        assert_eq!(tree_ids1.len(), 2);
        assert!(tree_ids1.contains(&1));
        assert!(tree_ids1.contains(&2));

        // Find the duplicate at (38.7749000, -123.4194000)
        let duplicate2 = duplicates.iter().find(|(lat, lon, _)| {
            (*lat - 38.7749000).abs() < 0.0000001 && (*lon + 123.4194000).abs() < 0.0000001
        });
        assert!(duplicate2.is_some());
        let (_, _, tree_ids2) = duplicate2.expect("Error getting duplicates.");
        assert_eq!(tree_ids2.len(), 2);
        assert!(tree_ids2.contains(&4));
        assert!(tree_ids2.contains(&5));
    }

    #[tokio::test]
    async fn test_merge_duplicates() {
        let (service, db) = setup().await;

        // Clear database
        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();
        db.execute_sql("DELETE FROM trees_images", &[])
            .await
            .unwrap();
        db.execute_sql("DELETE FROM comments", &[]).await.unwrap();
        db.execute_sql("DELETE FROM likes", &[]).await.unwrap();
        db.execute_sql("DELETE FROM observations", &[])
            .await
            .unwrap();

        // Create two duplicate trees
        let now = get_timestamp();

        let tree1 = Tree {
            id: 1,
            lat: 40.0,
            lon: 44.0,
            species: "Old Species".to_string(),
            height: Some(10.0),
            height_updated_at: now - 1000,
            updated_at: now - 1000,
            state: "healthy".to_string(),
            ..Default::default()
        };

        let tree2 = Tree {
            id: 2, // higher ID, so secondary
            lat: 40.0,
            lon: 44.0,
            species: "New Species".to_string(),
            height: Some(15.0),
            height_updated_at: now,
            updated_at: now,
            state: "healthy".to_string(),
            ..Default::default()
        };

        service.trees.add(&tree1).await.unwrap();
        service.trees.add(&tree2).await.unwrap();

        // Add a comment to the secondary tree
        service
            .comments
            .add(&crate::domain::comment::Comment {
                id: 1,
                tree_id: 2,
                added_by: 1,
                message: "Secondary comment".to_string(),
                added_at: now,
            })
            .await
            .unwrap();

        // Perform merge
        let merged = service.merge_duplicates(10).await.unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], (2, 1));

        // Verify main tree (ID 1)
        let main = service.trees.get(1).await.unwrap().unwrap();
        assert_eq!(main.species, "New Species");
        assert_eq!(main.height, Some(15.0));
        assert_eq!(main.comment_count, 1);

        // Verify secondary tree (ID 2)
        let secondary = service.trees.get(2).await.unwrap().unwrap();
        assert_eq!(secondary.state, "replaced");
        assert_eq!(secondary.replaced_by, Some(1));

        // Verify comments moved
        let comments = service.comments.find_by_tree(1).await.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].message, "Secondary comment");

        // --- Additional Test: Main tree is "gone" and secondary is "Unknown" species ---
        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let tree3 = Tree {
            id: 3,
            lat: 10.0,
            lon: 10.0,
            species: "Valid Species".to_string(),
            state: "gone".to_string(),
            updated_at: now - 100,
            ..Default::default()
        };

        let tree4 = Tree {
            id: 4,
            lat: 10.0,
            lon: 10.0,
            species: "Unknown".to_string(),
            state: "sick".to_string(),
            updated_at: now,
            ..Default::default()
        };

        service.trees.add(&tree3).await.unwrap();
        service.trees.add(&tree4).await.unwrap();

        service.merge_duplicates(10).await.unwrap();

        let main2 = service.trees.get(3).await.unwrap().unwrap();
        assert_eq!(main2.state, "sick"); // Should take "sick" from tree4, ignoring "gone" from tree3
        assert_eq!(main2.species, "Valid Species"); // Should take "Valid Species" from tree3, ignoring "Unknown" from tree4
    }

    #[tokio::test]
    async fn test_remap_osm_duplicates() {
        let (service, db) = setup().await;

        // Clear database
        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();
        db.execute_sql("DELETE FROM osm_trees", &[]).await.unwrap();

        // Create two local trees that were merged
        // Tree 1 is main, Tree 2 was merged into it
        let tree1 = Tree {
            id: 1,
            lat: 40.0,
            lon: 44.0,
            osm_id: Some(101), // points to deleted OSM node
            state: "healthy".to_string(),
            ..Default::default()
        };

        let tree2 = Tree {
            id: 2,
            lat: 40.0,
            lon: 44.0,
            osm_id: Some(102), // points to visible OSM node
            state: "replaced".to_string(),
            replaced_by: Some(1),
            ..Default::default()
        };

        service.trees.add(&tree1).await.unwrap();
        service.trees.add(&tree2).await.unwrap();

        // Setup OSM nodes in cache
        let osm101 = crate::domain::osm::OsmTreeRecord {
            id: 101,
            visible: false, // Deleted on OSM
            ..Default::default()
        };

        let osm102 = crate::domain::osm::OsmTreeRecord {
            id: 102,
            visible: true, // Remains on OSM
            ..Default::default()
        };

        service.osm_trees.add(&osm101).await.unwrap();
        service.osm_trees.add(&osm102).await.unwrap();

        // Perform remapping
        let remapped = service.remap_osm_duplicates().await.unwrap();
        assert_eq!(remapped.len(), 1);
        assert_eq!(remapped[0], (1, 2));

        // Verify OSM IDs swapped
        let t1 = service.trees.get(1).await.unwrap().unwrap();
        let t2 = service.trees.get(2).await.unwrap().unwrap();

        assert_eq!(t1.osm_id, Some(102));
        assert_eq!(t2.osm_id, Some(101));
    }
}
