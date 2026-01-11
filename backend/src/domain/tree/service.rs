use super::schemas::*;
use crate::domain::comment::CommentService;
use crate::domain::file::FileRepository;
use crate::domain::prop::{PropRecord, PropRepository};
use crate::domain::tree::{Tree, TreeRepository};
use crate::domain::user::UserRepository;
use crate::infra::config::Config;
use crate::infra::database::Database;
use crate::infra::nominatim::NominatimClient;
use crate::infra::queue::Queue;
use crate::services::queue_consumer::{AddPhotoMessage, UpdateTreeAddressMessage};
use crate::services::*;
use crate::types::*;
use crate::utils::osm_round_coord;
use crate::utils::{fix_circumference, get_timestamp, get_unique_id};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;

const DISTANCE: f64 = 0.1; // 10 cm

pub struct TreeService {
    db: Arc<Database>,
    comments: Arc<CommentService>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    queue: Arc<Queue>,
    files: Arc<FileRepository>,
    props: Arc<PropRepository>,
    nominatim: Arc<NominatimClient>,
    bot_user_id: u64,
}

impl TreeService {
    pub async fn get_duplicates(&self) -> Result<DuplicatesResponse> {
        // Get all trees from the database
        let trees = self.trees.all().await?;

        // HashMap to store coordinate -> tree_ids mapping
        let mut location_map: HashMap<String, Vec<String>> = HashMap::new();

        // Process each tree
        for tree in trees {
            if !Self::is_visible(&tree) {
                continue; // Skip trees that are not visible
            }

            // Round coordinates using OSM standard (7 decimal places)
            let rounded_lat = osm_round_coord(tree.lat);
            let rounded_lon = osm_round_coord(tree.lon);

            // Create location key
            let location_key = format!("{rounded_lat},{rounded_lon}");

            // Add tree ID to the location
            location_map
                .entry(location_key)
                .or_default()
                .push(tree.id.to_string());
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

        debug!("Returning {} duplicate trees.", duplicates.len());

        Ok(DuplicatesResponse::new(duplicates))
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

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            trees.retain(|t| query.r#match(t, user_id));
        } else {
            trees.retain(Self::is_visible);
        }

        debug!("Filtered {} trees.", trees.len());

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
                state: "gone".to_string(),
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

        let updated = self
            .trees
            .update(
                &Tree {
                    circumference: Some(value),
                    ..tree.clone()
                },
                user_id,
            )
            .await?;

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

        let updated = self
            .trees
            .update(
                &Tree {
                    diameter: Some(value),
                    ..tree.clone()
                },
                user_id,
            )
            .await?;

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

        let updated = self
            .trees
            .update(
                &Tree {
                    height: Some(value),
                    ..tree.clone()
                },
                user_id,
            )
            .await?;

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
            circumference: match fix_circumference(req.circumference) {
                Some(value) => Some(value),
                None => old.circumference,
            },
            diameter: match req.diameter {
                Some(value) => Some(value),
                None => old.diameter,
            },
            state: match req.state {
                Some(value) => value,
                None => old.state,
            },
            added_at: old.added_at,
            updated_at: now,
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
        self.db.get_species_mismatch(count, skip).await
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
            .update(
                &Tree {
                    address: Some(address),
                    ..tree.clone()
                },
                self.bot_user_id,
            )
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
        if tree.state == "gone" {
            return false;
        }

        if tree.species.to_lowercase().contains("error") {
            return false;
        }

        true
    }
}

impl Locatable for TreeService {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        Ok(Self {
            db: locator.get::<Database>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            queue: locator.get::<Queue>()?,
            files: locator.get::<FileRepository>()?,
            props: locator.get::<PropRepository>()?,
            comments: locator.get::<CommentService>()?,
            nominatim: locator.get::<NominatimClient>()?,
            bot_user_id: config.bot_user_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn setup() -> Arc<TreeService> {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let locator = Locator::new();

        locator
            .get::<TreeService>()
            .expect("Error creating TreeService")
    }

    #[tokio::test]
    async fn test_get_trees() {
        let trees = setup();

        let request = GetTreesRequest {
            n: 0.0,
            e: 0.0,
            s: 0.0,
            w: 0.0,
            search: None,
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
}
