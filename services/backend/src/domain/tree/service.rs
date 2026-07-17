use super::schemas::*;
use crate::domain::comment::{CommentRepository, CommentService};
use crate::domain::like::LikeRepository;
use crate::domain::observation::ObservationRepository;
use crate::domain::osm::OsmTreeRepository;
use crate::domain::prop::{PropRecord, PropRepository};
use crate::domain::tree::{Tree, TreeRepository};
use crate::domain::tree_image::TreeImageRepository;
use crate::domain::user::UserRepository;
use crate::infra::database::{Database, Value};
use crate::infra::nominatim::NominatimClient;
use crate::infra::queue::Queue;
use crate::services::queue_consumer::{AddPhotoMessage, UpdateTreeAddressMessage};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp, get_unique_id};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;

const DISTANCE: f64 = 0.1; // 10 cm
const EXCLUDED_MERGE_PROPS: &[&str] = &["replaced_by", "replaces"];

pub struct TreeService {
    db: Arc<Database>,
    comments: Arc<CommentService>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    queue: Arc<Queue>,
    files: Arc<TreeImageRepository>,
    props: Arc<PropRepository>,
    likes: Arc<LikeRepository>,
    observations: Arc<ObservationRepository>,
    comments_repo: Arc<CommentRepository>,
    osm_trees: Arc<OsmTreeRepository>,
    nominatim: Arc<NominatimClient>,
    bot_user_id: u64,
}

impl TreeService {
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

    pub async fn add_trees(&self, req: AddTreeRequest) -> Result<Vec<Tree>> {
        let now = get_timestamp();

        let mut trees: Vec<Tree> = Vec::new();

        for point in &req.points {
            let tree = self.add_tree(point.lat, point.lon, &req, now).await?;
            trees.push(tree);
        }

        Ok(trees)
    }

    pub async fn get_new_trees(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        self.trees.get_recently_created(count, skip).await
    }

    pub async fn get_defaults(&self, user_id: u64) -> Result<NewTreeDefaultsResponse> {
        match self.trees.get_last_by_user(user_id).await? {
            Some(tree) => Ok(tree.into()),

            None => Ok(NewTreeDefaultsResponse {
                species: None,
                notes: None,
                height: Some(0.0),
                circumference: Some(0.0),
                diameter: Some(0.0),
                state: Some("healthy".to_string()),
            }),
        }
    }

    pub async fn get_tree(&self, id: u64) -> Result<Tree> {
        self.trees.get(id).await?.ok_or(Error::TreeNotFound)
    }

    pub async fn get_trees(&self, request: &GetTreesRequest, user_id: u64) -> Result<Vec<Tree>> {
        debug!("Going to find trees.");

        let mut trees = self.trees.get_by_bounds(request.into()).await?;

        debug!("Found {} trees.", trees.len());

        trees.retain(Self::is_visible);

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            for tree in trees.iter_mut() {
                if !query.r#match(tree, user_id) {
                    tree.state = "placeholder".to_string();
                    tree.circumference = Some(0.0);
                }
            }
        }

        if let Some(zoom) = request.zoom {
            if zoom < 15.0 {
                trees.retain(|t| t.state != "placeholder");
            }
        }

        debug!("Found {} visible trees.", trees.len());

        Ok(trees)
    }

    pub async fn get_stats(&self) -> Result<TreeStats> {
        let count = self.trees.count().await?;

        info!("Serving tree stats (health check ok).");

        Ok(TreeStats { count })
    }

    pub async fn get_recently_updated(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        self.trees.get_recently_updated(count, skip).await
    }

    pub async fn get_liked_trees(&self, user_id: u64, count: u64, skip: u64) -> Result<Vec<Tree>> {
        self.trees.get_liked_by_user(user_id, count, skip).await
    }

    pub async fn move_tree(&self, tree_id: u64, user_id: u64, lat: f64, lon: f64) -> Result<()> {
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        self.trees.r#move(&tree, lat, lon, user_id).await?;

        info!("Tree {tree_id} moved to ({lat},{lon})");

        self.users.increment_update_count(user_id).await?;

        Ok(())
    }

    pub async fn replace_tree(&self, req: ReplaceTreeRequest) -> Result<Vec<Tree>> {
        let now = get_timestamp();

        let old = self.trees.get(req.id).await?.ok_or(Error::TreeNotFound)?;

        let new = Tree {
            id: get_unique_id()?,
            lat: old.lat,
            lon: old.lon,
            species: req.species.clone(),
            notes: req.notes.clone(),
            height: req.height,
            circumference: fix_circumference(req.circumference),
            diameter: req.diameter,
            state: req.state.to_string(),
            added_at: now,
            added_by: req.user_id,
            updated_at: now,
            updated_by: req.user_id,
            year: req.year,
            address: old.address.clone(),
            replaces: Some(old.id),
            ..Default::default()
        };

        self.trees.add(&new).await?;
        self.trees.replace(old.id, new.id, req.user_id).await?;
        self.schedule_files(new.id, req.files.clone()).await?;
        self.users.increment_tree_count(req.user_id).await?;

        info!(
            "Tree {} added as a replacement for tree {} by {}",
            new.id, req.id, req.user_id
        );

        let trees: Vec<Tree> = vec![
            Tree {
                state: "replaced".to_string(),
                replaced_by: Some(new.id),
                ..old
            },
            new,
        ];

        Ok(trees)
    }

    pub async fn update_circumference(
        &self,
        tree_id: u64,
        value: f64,
        user_id: u64,
    ) -> Result<Tree> {
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;
        let mut new_tree = tree.clone();
        new_tree.circumference = Some(value);
        new_tree.circumference_updated_at = get_timestamp();

        let updated = self.trees.update(&new_tree, user_id).await?;

        self.users.increment_update_count(user_id).await?;

        info!("Circumference for tree {tree_id} changed to {value} by {user_id}.");

        Ok(updated)
    }

    pub async fn update_diameter(&self, tree_id: u64, value: f64, user_id: u64) -> Result<Tree> {
        // Validate that tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        if tree.diameter == Some(value) {
            // If the diameter is the same, we still need to store the property update.
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "diameter".to_string(),
                    value: value.to_string(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        let mut new_tree = tree.clone();
        new_tree.diameter = Some(value);
        new_tree.diameter_updated_at = get_timestamp();

        let updated = self.trees.update(&new_tree, user_id).await?;

        self.users.increment_update_count(user_id).await?;

        info!("Diameter for tree {tree_id} changed to {value} by {user_id}.");

        Ok(updated)
    }

    pub async fn update_height(&self, tree_id: u64, value: f64, user_id: u64) -> Result<Tree> {
        // Make sure the tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        if tree.height == Some(value) {
            // If the height is the same, we still need to store the property update.
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "height".to_string(),
                    value: value.to_string(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        let mut new_tree = tree.clone();
        new_tree.height = Some(value);
        new_tree.height_updated_at = get_timestamp();

        let updated = self.trees.update(&new_tree, user_id).await?;

        self.users.increment_update_count(user_id).await?;

        info!("Height for tree {tree_id} changed to {value} by {user_id}.");

        Ok(updated)
    }

    pub async fn update_location(
        &self,
        tree_id: u64,
        lat: f64,
        lon: f64,
        user_id: u64,
    ) -> Result<Tree> {
        // Make sure the tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        if tree.lat == lat && tree.lon == lon {
            // If the height is the same, we still need to store the property update.
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "location".to_string(),
                    value: format!("{lat},{lon}").to_string(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        let updated = self
            .trees
            .update(
                &Tree {
                    lat,
                    lon,
                    ..tree.clone()
                },
                user_id,
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!("Location for tree {tree_id} changed to {lat},{lon} by {user_id}.");

        Ok(updated)
    }

    pub async fn update_state(
        &self,
        tree_id: u64,
        value: String,
        user_id: u64,
        comment: Option<String>,
    ) -> Result<Tree> {
        // Validate that tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        // The update method only logs changes, but for this explicit handler
        // we need to save all data, even if no changes were made (confirm the value).
        if tree.state == value {
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "state".to_string(),
                    value: value.clone(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        let updated = self
            .trees
            .update(
                &Tree {
                    state: value.clone(),
                    ..tree.clone()
                },
                user_id,
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        // Add comment if provided
        if let Some(comment_text) = comment {
            if !comment_text.trim().is_empty() {
                self.comments
                    .add_comment(tree_id, user_id, &comment_text)
                    .await?;
            }
        }

        info!("State for tree {tree_id} changed to {value} by {user_id}.");

        Ok(updated)
    }

    pub async fn update_tree(&self, req: UpdateTreeRequest) -> Result<Tree> {
        let now = get_timestamp();

        let old = match self.trees.get(req.id).await? {
            Some(tree) => tree,
            None => return Err(Error::TreeNotFound),
        };

        let new = Tree {
            id: req.id,
            osm_id: old.osm_id,
            species: req.species.unwrap_or(old.species),
            lat: req.lat.unwrap_or(old.lat),
            lon: req.lon.unwrap_or(old.lon),
            notes: match req.notes {
                Some(value) => Some(value),
                None => old.notes,
            },
            height: match req.height {
                Some(value) => Some(value),
                None => old.height,
            },
            height_updated_at: if req.height.is_some() {
                now
            } else {
                old.height_updated_at
            },
            circumference: match fix_circumference(req.circumference) {
                Some(value) => Some(value),
                None => old.circumference,
            },
            circumference_updated_at: if req.circumference.is_some() {
                now
            } else {
                old.circumference_updated_at
            },
            diameter: match req.diameter {
                Some(value) => Some(value),
                None => old.diameter,
            },
            diameter_updated_at: if req.diameter.is_some() {
                now
            } else {
                old.diameter_updated_at
            },
            state: match req.state {
                Some(value) => value,
                None => old.state,
            },
            added_at: old.added_at,
            updated_at: now,
            updated_by: req.user_id,
            added_by: old.added_by,
            thumbnail_id: old.thumbnail_id,
            year: match req.year {
                Some(value) => Some(value),
                None => old.year,
            },
            address: match req.address {
                Some(value) => Some(value),
                None => old.address,
            },
            ..old
        };

        info!("Updating tree: {new:?}");

        let updated = self.trees.update(&new, req.user_id).await?;

        if new.address.is_none() {
            self.schedule_address_update(new.id).await?;
        }

        self.users.increment_update_count(req.user_id).await?;

        Ok(updated)
    }

    pub async fn update_thumbnail(&self, tree_id: u64, file_id: u64, user_id: u64) -> Result<()> {
        self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        let file = self.files.get(file_id).await?.ok_or(Error::FileNotFound)?;

        self.trees
            .update_thumbnail(tree_id, file.small_id, user_id)
            .await?;

        self.users.increment_update_count(user_id).await?;

        info!("Thumbnail for tree {tree_id} changed to {file_id} by {user_id}.",);

        Ok(())
    }

    pub async fn get_top_by_height(&self) -> Result<Vec<Tree>> {
        self.trees.get_top_height(100).await
    }

    pub async fn get_top_by_diameter(&self) -> Result<Vec<Tree>> {
        self.trees.get_top_diameter(100).await
    }

    pub async fn get_top_by_circumference(&self) -> Result<Vec<Tree>> {
        self.trees.get_top_circumference(100).await
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
                self.comments_repo
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

    // Add photos to an existing tree.
    //
    // The files were uploaded previously and their source files
    // are stored in the main file storage.
    //
    // We don't process the files here (resize etc), just send
    // the requests to the queue.
    pub async fn add_tree_photos(
        &self,
        tree_id: u64,
        user_id: u64,
        files: Vec<String>,
    ) -> Result<()> {
        info!(
            "Received {} files from user {user_id} for tree {tree_id}.",
            files.len(),
        );

        for file in &files {
            let file_id: u64 = file.parse().map_err(|e| {
                error!("Error processing file ID {file}: {e}");
                Error::BadRequest
            })?;

            let message = AddPhotoMessage { tree_id, file_id };
            self.queue.push(&message.encode()).await?;
        }

        Ok(())
    }

    pub async fn get_mismatching_species(&self, count: u64, skip: u64) -> Result<Vec<Tree>> {
        let rows = self
            .db
            .fetch_sql(
                "SELECT * FROM trees WHERE state <> 'gone' AND species <> 'Unknown species' AND species <> 'Unknown' AND species NOT IN (SELECT name FROM species) LIMIT ?1 OFFSET ?2",
                &[Value::from(count), Value::from(skip)],
            )
            .await?;

        let mut records = Vec::new();

        for row in rows {
            if let Ok(packed) = Tree::from_attributes(&row) {
                records.push(packed);
            }
        }

        Ok(records)
    }

    // Update the street address of a tree based on its coordinates.
    //
    // This is triggered whenever we change a tree coordinates using the UI.
    pub async fn update_tree_address(&self, tree_id: u64) -> Result<()> {
        if let Ok(Some(tree)) = self.trees.get(tree_id).await {
            self.update_single_tree_address(&tree).await?;
        }

        Ok(())
    }

    // Update addresses of all known trees.
    //
    // This is used via CLI as a maintenance task.
    pub async fn update_all_tree_addresses(&self) -> Result<()> {
        let trees = self.trees.get_with_no_address().await?;

        for tree in trees {
            self.update_single_tree_address(&tree).await?;
            self.sleep();
        }

        Ok(())
    }

    // Nominotim's usage policy requires request rate of at most 1 rps.
    // Let's wait twice as much.
    // This is a one time manually ran operation so we aren't in a hurry.
    // @docs https://operations.osmfoundation.org/policies/nominatim/
    fn sleep(&self) {
        let dur = std::time::Duration::from_secs(2);
        std::thread::sleep(dur);
    }

    async fn update_single_tree_address(&self, tree: &Tree) -> Result<()> {
        let address = match self
            .nominatim
            .get_street_address(tree.lat, tree.lon)
            .await?
        {
            Some(value) => value,

            None => {
                warn!("No address for tree {}.", tree.id);
                return Ok(());
            }
        };

        info!("Updating tree {} address to: {}", tree.id, address);

        self.trees
            .update_address(tree.id, &address, self.bot_user_id)
            .await?;

        Ok(())
    }

    // Send files for attaching to the new tree.
    async fn schedule_files(&self, tree_id: u64, files: Vec<String>) -> Result<()> {
        for file in files {
            match file.parse::<u64>() {
                Ok(file_id) => {
                    info!("Scheduling file {file_id} for tree {tree_id}");

                    let message = AddPhotoMessage { tree_id, file_id };
                    self.queue.push(&message.encode()).await?;
                }

                Err(e) => {
                    error!(
                        "Error parsing file ID '{file}': {e} -- cannot attach to tree {tree_id}."
                    );
                }
            }
        }

        Ok(())
    }

    // Add a single tree.
    async fn add_tree(&self, lat: f64, lon: f64, req: &AddTreeRequest, now: u64) -> Result<Tree> {
        let tree = Tree {
            id: get_unique_id()?,
            lat,
            lon,
            species: req.species.clone(),
            notes: req.notes.clone(),
            height: req.height,
            circumference: fix_circumference(req.circumference),
            diameter: req.diameter,
            state: req.state.to_string(),
            added_at: now,
            added_by: req.user_id,
            updated_at: now,
            updated_by: req.user_id,
            thumbnail_id: None,
            year: req.year,
            address: req.address.clone(),
            ..Default::default()
        };

        debug!(
            "Adding tree at ({}, {}) with species '{}'.",
            lat, lon, tree.species
        );

        if self.exists_with_coordinates(tree.lat, tree.lon).await? {
            return Err(Error::DuplicateTree);
        }

        self.trees.add(&tree).await?;

        self.schedule_address_update(tree.id).await?;
        self.schedule_files(tree.id, req.files.clone()).await?;
        self.users.increment_tree_count(req.user_id).await?;

        Ok(tree)
    }

    async fn exists_with_coordinates(&self, lat: f64, lon: f64) -> Result<bool> {
        for tree in self.trees.get_close(lat, lon, DISTANCE).await? {
            if tree.state != "gone" {
                debug!(
                    "Tree {} already exists at coordinates ({}, {})",
                    tree.id, lat, lon
                );
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn schedule_address_update(&self, tree_id: u64) -> Result<()> {
        let msg = UpdateTreeAddressMessage { id: tree_id };
        self.queue.push(&msg.encode()).await?;

        info!("Scheduled address update for tree {tree_id}");

        Ok(())
    }

    fn is_visible(tree: &Tree) -> bool {
        if tree.state == "replaced" {
            return false;
        }

        if tree.species.to_lowercase().contains("error") {
            return false;
        }

        true
    }
}

impl Injectable for TreeService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            comments: Arc::new(ctx.build::<CommentService>()?),
            trees: Arc::new(ctx.build::<TreeRepository>()?),
            users: Arc::new(ctx.build::<UserRepository>()?),
            queue: ctx.queue(),
            files: Arc::new(ctx.build::<TreeImageRepository>()?),
            props: Arc::new(ctx.build::<PropRepository>()?),
            likes: Arc::new(ctx.build::<LikeRepository>()?),
            observations: Arc::new(ctx.build::<ObservationRepository>()?),
            comments_repo: Arc::new(ctx.build::<CommentRepository>()?),
            osm_trees: Arc::new(ctx.build::<OsmTreeRepository>()?),
            nominatim: Arc::new(ctx.build::<NominatimClient>()?),
            bot_user_id: ctx.config().bot_user_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;
    use crate::utils::osm_round_coord;
    use std::collections::HashMap;

    async fn setup() -> Arc<TreeService> {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<TreeService>()
                .expect("Error creating TreeService"),
        )
    }

    #[tokio::test]
    async fn test_get_trees() {
        let trees = setup().await;

        let request = GetTreesRequest {
            n: 0.0,
            e: 0.0,
            s: 0.0,
            w: 0.0,
            search: None,
            zoom: None,
        };

        trees
            .get_trees(&request, 0)
            .await
            .expect("Error getting trees.");
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
    async fn test_get_mismatching_species() {
        let service = setup().await;

        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .expect("Error clearing trees.");

        service
            .db
            .execute_sql("DELETE FROM species", &[])
            .await
            .expect("Error clearing species.");

        service
            .db
            .execute_sql("INSERT INTO trees (id, lat, lon, species, state, added_at, updated_at, updated_by, added_by) VALUES (1, 40.1, 44.1, 'Fake Species', 'healthy', 0, 0, 1, 1)", &[])
            .await
            .expect("Error adding tree.");

        let res = service
            .get_mismatching_species(10, 0)
            .await
            .expect("Error getting mismatch.");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].species, "Fake Species");
    }

    #[tokio::test]
    async fn test_merge_duplicates() {
        let service = setup().await;

        // Clear database
        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .unwrap();
        service
            .db
            .execute_sql("DELETE FROM trees_images", &[])
            .await
            .unwrap();
        service
            .db
            .execute_sql("DELETE FROM comments", &[])
            .await
            .unwrap();
        service
            .db
            .execute_sql("DELETE FROM likes", &[])
            .await
            .unwrap();
        service
            .db
            .execute_sql("DELETE FROM observations", &[])
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
            .add_comment(2, 1, "Secondary comment")
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
        let comments = service.comments.get_tree_comments(1).await.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].message, "Secondary comment");

        // --- Additional Test: Main tree is "gone" and secondary is "Unknown" species ---
        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .unwrap();
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
        let service = setup().await;

        // Clear database
        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .unwrap();
        service
            .db
            .execute_sql("DELETE FROM osm_trees", &[])
            .await
            .unwrap();

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
