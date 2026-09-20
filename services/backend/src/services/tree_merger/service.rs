use super::clustering::{clusterize, filter_by_states, find_candidates};
use crate::domain::comment::CommentRepository;
use crate::domain::like::LikeRepository;
use crate::domain::observation::ObservationRepository;
use crate::domain::osm::OsmTreeRepository;
use crate::domain::prop::{PropRecord, PropRepository};
use crate::domain::tree::{TreeLocation, TreeRepository, TreeState};
use crate::domain::tree_image::TreeImageRepository;
use crate::services::{Context, Injectable};
use crate::types::*;
use log::{debug, info};
use std::collections::HashSet;
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
    /// Finds pairs of trees where a gone or stump tree sits within `proximity`
    /// meters of an alive tree. The alive tree is always the merge target.
    pub async fn find_auto_merge_candidates(
        &self,
        proximity: f64,
    ) -> Result<Vec<(TreeLocation, TreeLocation)>> {
        let trees = self.trees.get_lightweight_locations().await?;
        let trees = filter_by_states(
            trees,
            &[TreeState::Alive, TreeState::Gone, TreeState::Stump],
        );

        let clustered = clusterize(trees, proximity);

        let candidates = find_candidates(
            &clustered,
            proximity,
            &[TreeState::Gone, TreeState::Stump],
            &[TreeState::Alive],
        );

        debug!("Found {} auto-merge candidates.", candidates.len());

        Ok(candidates)
    }

    /// Finds pairs of alive trees within `proximity` meters of each other.
    /// Unordered pairs are deduplicated and oriented with the lower id as the
    /// target (main) tree.
    pub async fn find_manual_merge_candidates(
        &self,
        proximity: f64,
    ) -> Result<Vec<(TreeLocation, TreeLocation)>> {
        let trees = self.trees.get_lightweight_locations().await?;
        let trees = filter_by_states(trees, &[TreeState::Alive]);

        let clustered = clusterize(trees, proximity);

        let candidates = find_candidates(
            &clustered,
            proximity,
            &[TreeState::Alive],
            &[TreeState::Alive],
        );

        let mut seen = HashSet::new();
        let mut pairs = Vec::new();

        for (a, b) in candidates {
            let key = if a.id < b.id {
                (a.id, b.id)
            } else {
                (b.id, a.id)
            };

            if !seen.insert(key) {
                continue;
            }

            // Orient the lower id as the target (main) tree.
            let (from, to) = if a.id < b.id { (b, a) } else { (a, b) };
            pairs.push((from, to));
        }

        debug!("Found {} manual-merge candidates.", pairs.len());

        Ok(pairs)
    }

    /// Links a tree to its replacement without merging any properties.
    ///
    /// Marks the `from` tree as replaced and points its `replaced_by` to `to`.
    /// Fails if the tree has already been replaced.
    pub async fn link_replaced_tree(&self, from_id: u64, to_id: u64) -> Result<()> {
        let trees = self.trees.get_multiple(&[from_id, to_id]).await?;

        let Some(from) = trees.iter().find(|t| t.id == from_id) else {
            return Err(Error::TreeNotFound);
        };

        let Some(to) = trees.iter().find(|t| t.id == to_id) else {
            return Err(Error::TreeNotFound);
        };

        if from.state == TreeState::Replaced || from.replaced_by.is_some() {
            return Err(Error::BadRequestMessage(format!(
                "Tree {from_id} is already replaced."
            )));
        }

        if to.state != TreeState::Alive {
            return Err(Error::BadRequestMessage(format!(
                "Tree {to_id} is not alive."
            )));
        }

        self.trees
            .mark_as_merged(from_id, to_id, self.bot_user_id)
            .await?;

        info!("Tree {} marked as replaced by {}.", from_id, to_id);

        Ok(())
    }

    /// Merges the `from` tree into the `to` tree.
    ///
    /// The source must not already be replaced, and the target must be alive.
    pub async fn merge_pair(&self, from_id: u64, to_id: u64) -> Result<Vec<(u64, u64)>> {
        let trees = self.trees.get_multiple(&[from_id, to_id]).await?;

        let Some(from) = trees.iter().find(|t| t.id == from_id).cloned() else {
            return Ok(Vec::new());
        };

        let Some(to) = trees.iter().find(|t| t.id == to_id).cloned() else {
            return Ok(Vec::new());
        };

        if from.state == TreeState::Replaced || to.state != TreeState::Alive {
            return Ok(Vec::new());
        }

        let all = vec![to.clone(), from.clone()];
        let mut merged_tree = to.clone();

        // (4) Should merge all props, latest takes precedence
        // scalar props: species, height, circumference, diameter, notes, year, address

        // Collect all trees sorted by updated_at DESC to find latest values
        let mut latest_first = all.clone();
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
        if let Some(t) = latest_first.iter().find(|t| t.state != TreeState::Gone) {
            merged_tree.state = t.state;
        }

        // Merge height
        if let Some(t) = all
            .iter()
            .filter(|t| t.height.is_some() && t.height.unwrap_or(0.0) > 0.0)
            .max_by_key(|t| t.height_updated_at)
        {
            merged_tree.height = t.height;
            merged_tree.height_updated_at = t.height_updated_at;
        }

        // Merge circumference
        if let Some(t) = all
            .iter()
            .filter(|t| t.circumference.is_some() && t.circumference.unwrap_or(0.0) > 0.0)
            .max_by_key(|t| t.circumference_updated_at)
        {
            merged_tree.circumference = t.circumference;
            merged_tree.circumference_updated_at = t.circumference_updated_at;
        }

        // Merge diameter
        if let Some(t) = all
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
        merged_tree.images_updated_at = all.iter().map(|t| t.images_updated_at).max().unwrap_or(0);
        merged_tree.observations_updated_at = all
            .iter()
            .map(|t| t.observations_updated_at)
            .max()
            .unwrap_or(0);

        // Merge thumbnail_id: if main has none, take latest available
        if merged_tree.thumbnail_id.is_none() {
            if let Some(t) = all
                .iter()
                .filter(|t| t.thumbnail_id.is_some())
                .max_by_key(|t| t.images_updated_at)
            {
                merged_tree.thumbnail_id = t.thumbnail_id;
            }
        }

        // Update main tree with merged values
        self.trees.update(&merged_tree, self.bot_user_id).await?;

        // (5) Should move all photos into the main tree
        self.files.reassign_all(from.id, to.id).await?;

        // Move comments
        self.comments.reassign_all(from.id, to.id).await?;

        // Move likes
        self.likes.reassign_all(from.id, to.id).await?;

        // Move observations
        self.observations.reassign_all(from.id, to.id).await?;

        // Move props history
        let secondary_props = self.props.find_by_tree(from.id).await?;

        let prop_ids_to_move: Vec<u64> = secondary_props
            .into_iter()
            .filter(|p| {
                if EXCLUDED_MERGE_PROPS.contains(&p.name.as_str()) {
                    return false;
                }

                if p.name == "state" && p.value == TreeState::Replaced.as_str() {
                    return false;
                }

                true
            })
            .map(|p| p.id)
            .collect();

        self.props.update_tree_id(prop_ids_to_move, to.id).await?;

        // Add audit trail entry
        self.props
            .add(&PropRecord {
                tree_id: to.id,
                name: "merged_from".to_string(),
                value: from.id.to_string(),
                added_by: self.bot_user_id,
                ..Default::default()
            })
            .await?;

        // (6) Should mark merged trees as replaced, with replaced_by pointing to the new tree.
        self.trees
            .mark_as_merged(from.id, to.id, self.bot_user_id)
            .await?;

        info!("Tree {} merged into {}.", from.id, to.id);

        // Recalculate stats for the main tree
        self.trees.recalculate_stats(to.id).await?;

        Ok(vec![(from.id, to.id)])
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
    use crate::domain::tree::{Tree, TreeState};
    use crate::infra::database::Database;
    use crate::services::tree_merger::DEFAULT_PROXIMITY_METERS;
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

    async fn run_auto_merge(service: &TreeMergerService, limit: usize) -> Vec<(u64, u64)> {
        let candidates = service
            .find_auto_merge_candidates(DEFAULT_PROXIMITY_METERS)
            .await
            .unwrap();

        let mut merged = Vec::new();

        for (from, to) in candidates.into_iter().take(limit) {
            merged.extend(service.merge_pair(from.id, to.id).await.unwrap());
        }

        merged
    }

    #[test]
    fn test_coordinate_rounding() {
        // Test the coordinate rounding function (OSM standard 7 decimal places)
        assert_eq!(osm_round_coord(40.1813891), 40.1813891);
        assert_eq!(osm_round_coord(40.18138999), 40.1813900);
        assert_eq!(osm_round_coord(-123.4194001), -123.4194001);
        assert_eq!(osm_round_coord(-123.41940009), -123.4194001);
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

        // Create two duplicate trees: the secondary is gone, the main is alive.
        let now = get_timestamp();

        let tree1 = Tree {
            id: 1,
            lat: 40.0,
            lon: 44.0,
            species: "Old Species".to_string(),
            height: Some(10.0),
            height_updated_at: now - 1000,
            updated_at: now - 1000,
            state: TreeState::Alive,
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
            state: TreeState::Gone,
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
        let merged = run_auto_merge(&service, 10).await;
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], (2, 1));

        // Verify main tree (ID 1)
        let main = service.trees.get(1).await.unwrap().unwrap();
        assert_eq!(main.species, "New Species");
        assert_eq!(main.height, Some(15.0));
        assert_eq!(main.comment_count, 1);

        // Verify secondary tree (ID 2)
        let secondary = service.trees.get(2).await.unwrap().unwrap();
        assert_eq!(secondary.state, TreeState::Replaced);
        assert_eq!(secondary.replaced_by, Some(1));

        // Verify comments moved
        let comments = service.comments.find_by_tree(1).await.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].message, "Secondary comment");

        // --- Additional Test: the alive tree becomes the main tree ---
        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let tree3 = Tree {
            id: 3,
            lat: 10.0,
            lon: 10.0,
            species: "Valid Species".to_string(),
            state: TreeState::Gone,
            updated_at: now - 100,
            ..Default::default()
        };

        let tree4 = Tree {
            id: 4,
            lat: 10.0,
            lon: 10.0,
            species: "Unknown".to_string(),
            state: TreeState::Alive,
            updated_at: now,
            ..Default::default()
        };

        service.trees.add(&tree3).await.unwrap();
        service.trees.add(&tree4).await.unwrap();

        let merged = run_auto_merge(&service, 10).await;
        assert_eq!(merged, vec![(3, 4)]);

        let main2 = service.trees.get(4).await.unwrap().unwrap();
        assert_eq!(main2.state, TreeState::Alive); // Should take alive from tree4, ignoring "gone" from tree3
        assert_eq!(main2.species, "Valid Species"); // Should take "Valid Species" from tree3, ignoring "Unknown" from tree4

        let secondary2 = service.trees.get(3).await.unwrap().unwrap();
        assert_eq!(secondary2.state, TreeState::Replaced);
        assert_eq!(secondary2.replaced_by, Some(4));
    }

    #[tokio::test]
    async fn test_merge_pair_uses_target_as_main() {
        let (service, db) = setup().await;

        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let now = get_timestamp();

        let lower = Tree {
            id: 5,
            lat: 40.0,
            lon: 44.0,
            species: "Lower".to_string(),
            state: TreeState::Alive,
            updated_at: now,
            ..Default::default()
        };

        let higher = Tree {
            id: 10,
            lat: 40.0,
            lon: 44.0,
            species: "Higher".to_string(),
            state: TreeState::Alive,
            updated_at: now,
            ..Default::default()
        };

        service.trees.add(&lower).await.unwrap();
        service.trees.add(&higher).await.unwrap();

        // Manual candidates orient the lower id as the target.
        let candidates = service
            .find_manual_merge_candidates(DEFAULT_PROXIMITY_METERS)
            .await
            .unwrap();

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].0.id, 10);
        assert_eq!(candidates[0].1.id, 5);

        let merged = service.merge_pair(10, 5).await.unwrap();
        assert_eq!(merged, vec![(10, 5)]);

        let main = service.trees.get(5).await.unwrap().unwrap();
        assert_eq!(main.state, TreeState::Alive);

        let secondary = service.trees.get(10).await.unwrap().unwrap();
        assert_eq!(secondary.state, TreeState::Replaced);
        assert_eq!(secondary.replaced_by, Some(5));
    }

    #[tokio::test]
    async fn test_merge_pair_skips_replaced_endpoint() {
        let (service, db) = setup().await;

        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let alive = Tree {
            id: 1,
            lat: 40.0,
            lon: 44.0,
            state: TreeState::Alive,
            ..Default::default()
        };

        let replaced = Tree {
            id: 2,
            lat: 40.0,
            lon: 44.0,
            state: TreeState::Replaced,
            replaced_by: Some(1),
            ..Default::default()
        };

        service.trees.add(&alive).await.unwrap();
        service.trees.add(&replaced).await.unwrap();

        assert!(service.merge_pair(2, 1).await.unwrap().is_empty());
        assert!(service.merge_pair(1, 2).await.unwrap().is_empty());

        // Neither tree was modified.
        assert_eq!(
            service.trees.get(1).await.unwrap().unwrap().state,
            TreeState::Alive
        );
        assert_eq!(
            service.trees.get(2).await.unwrap().unwrap().state,
            TreeState::Replaced
        );
    }

    #[tokio::test]
    async fn test_link_replaced_tree() {
        let (service, db) = setup().await;

        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let alive = Tree {
            id: 1,
            lat: 40.0,
            lon: 44.0,
            state: TreeState::Alive,
            ..Default::default()
        };

        let gone = Tree {
            id: 2,
            lat: 40.0,
            lon: 44.0,
            state: TreeState::Gone,
            ..Default::default()
        };

        service.trees.add(&alive).await.unwrap();
        service.trees.add(&gone).await.unwrap();

        service.link_replaced_tree(2, 1).await.unwrap();

        let linked = service.trees.get(2).await.unwrap().unwrap();
        assert_eq!(linked.state, TreeState::Replaced);
        assert_eq!(linked.replaced_by, Some(1));

        // Linking the same tree again must fail.
        assert!(service.link_replaced_tree(2, 1).await.is_err());

        // Linking to a tree that is not alive must fail.
        assert!(service.link_replaced_tree(1, 2).await.is_err());
    }

    #[tokio::test]
    async fn test_auto_merge_state_policy() {
        let (service, db) = setup().await;

        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let states = [
            (1, TreeState::Alive),
            (2, TreeState::Gone),
            (3, TreeState::Stump),
            (4, TreeState::Dead),
            (5, TreeState::Error),
            (6, TreeState::Placeholder),
            (7, TreeState::Unknown),
            (8, TreeState::Replaced),
        ];

        for (id, state) in states {
            service
                .trees
                .add(&Tree {
                    id,
                    lat: 40.0,
                    lon: 44.0,
                    state,
                    ..Default::default()
                })
                .await
                .unwrap();
        }

        let candidates = service
            .find_auto_merge_candidates(DEFAULT_PROXIMITY_METERS)
            .await
            .unwrap();

        let mut pairs: Vec<(u64, u64)> = candidates
            .iter()
            .map(|(from, to)| (from.id, to.id))
            .collect();
        pairs.sort();

        assert_eq!(pairs, vec![(2, 1), (3, 1)]);

        let excluded = [
            TreeState::Dead,
            TreeState::Error,
            TreeState::Placeholder,
            TreeState::Unknown,
            TreeState::Replaced,
        ];

        for (from, to) in &candidates {
            assert!(!excluded.contains(&from.state));
            assert!(!excluded.contains(&to.state));
        }
    }

    #[tokio::test]
    async fn test_manual_merge_state_policy() {
        let (service, db) = setup().await;

        db.execute_sql("DELETE FROM trees", &[]).await.unwrap();

        let states = [
            (1, TreeState::Alive),
            (2, TreeState::Alive),
            (3, TreeState::Dead),
            (4, TreeState::Error),
            (5, TreeState::Placeholder),
            (6, TreeState::Unknown),
            (7, TreeState::Replaced),
            (8, TreeState::Gone),
            (9, TreeState::Stump),
        ];

        for (id, state) in states {
            service
                .trees
                .add(&Tree {
                    id,
                    lat: 40.0,
                    lon: 44.0,
                    state,
                    ..Default::default()
                })
                .await
                .unwrap();
        }

        let candidates = service
            .find_manual_merge_candidates(DEFAULT_PROXIMITY_METERS)
            .await
            .unwrap();

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].0.id, 2);
        assert_eq!(candidates[0].1.id, 1);

        let excluded = [
            TreeState::Dead,
            TreeState::Error,
            TreeState::Placeholder,
            TreeState::Unknown,
            TreeState::Replaced,
            TreeState::Gone,
            TreeState::Stump,
        ];

        for (from, to) in &candidates {
            assert!(!excluded.contains(&from.state));
            assert!(!excluded.contains(&to.state));
        }
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
            state: TreeState::Alive,
            ..Default::default()
        };

        let tree2 = Tree {
            id: 2,
            lat: 40.0,
            lon: 44.0,
            osm_id: Some(102), // points to visible OSM node
            state: TreeState::Replaced,
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
