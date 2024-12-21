//! Database client implementation for SQLite.
//!
//! Uses async-sqlite.
//!
//! Probably nee to split the core database code into a separate class,
//! and the queue implementation should also be separate, with its own database.
//!
//! @docs https://docs.rs/async-sqlite/latest/async_sqlite/

use crate::services::*;
use crate::types::*;
use crate::utils::{get_sqlite_path, get_timestamp, get_unique_id};
use async_sqlite::{JournalMode, Pool, PoolBuilder};
use async_trait::async_trait;
use log::{debug, error, info};
use std::cmp::Ordering;

const SUGGEST_WINDOW: u64 = 3600 * 24; // 24 hours

pub struct SqliteDatabase {
    pub pool: Pool,
}

impl SqliteDatabase {
    pub async fn new() -> Result<Self> {
        let path = get_sqlite_path()?;

        Ok(Self {
            pool: Self::create_pool(&path).await?,
        })
    }

    async fn create_pool(path: &str) -> Result<Pool> {
        match path {
            ":memory:" => Self::create_memory_pool().await,
            _ => Self::create_file_pool(path).await,
        }
    }

    async fn create_file_pool(path: &str) -> Result<Pool> {
        info!("Using SQLite database from {}.", path);

        let pool = match PoolBuilder::new()
            .path(path)
            .journal_mode(JournalMode::Wal)
            .open()
            .await
        {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {}", e);
                return Err(Error::DatabaseConnect);
            }
        };

        Ok(pool)
    }

    async fn create_memory_pool() -> Result<Pool> {
        info!("Using an in-memory SQLite database.");

        let pool = match PoolBuilder::new().num_conns(1).open().await {
            Ok(value) => value,
            Err(e) => {
                error!("Error connecting to the database: {}", e);
                return Err(Error::DatabaseConnect);
            }
        };

        Self::setup_memory_db(&pool).await?;

        Ok(pool)
    }

    async fn setup_memory_db(pool: &Pool) -> Result<()> {
        let script = include_str!("../../../dev/schema-sqlite.sql");
        Self::execute_pool(pool, script.to_string()).await?;

        debug!("Memory database initialized.");

        Ok(())
    }

    #[allow(unused)]
    pub async fn execute(&self, script: String) -> Result<()> {
        Self::execute_pool(&self.pool, script).await
    }

    async fn execute_pool(pool: &Pool, script: String) -> Result<()> {
        let res = pool.conn(move |conn| conn.execute_batch(&script)).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error executing SQL script: {}", e);
                Err(Error::DatabaseQuery)
            }
        }
    }

    async fn log_tree_changes(&self, old: &TreeRecord, new: &TreeRecord) -> Result<()> {
        debug!("Logging changes for tree {}.", new.id);

        if old.species != new.species {
            self.add_tree_prop(new.id, "species", &new.species).await?;
        }

        if old.osm_id != new.osm_id {
            let value = match new.osm_id {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "osm_id", &value).await?;
        }

        if old.lat != new.lat {
            self.add_tree_prop(new.id, "lat", &new.lat.to_string())
                .await?;
        }

        if old.lon != new.lon {
            self.add_tree_prop(new.id, "lon", &new.lon.to_string())
                .await?;
        }

        if old.species != new.species {
            self.add_tree_prop(new.id, "species", &new.species).await?;
        }

        if old.notes != new.notes {
            let value = match &new.notes {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "notes", &value).await?;
        }

        if old.height != new.height {
            let value = match new.height {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "height", &value).await?;
        }

        if old.circumference != new.circumference {
            let value = match new.circumference {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "circumference", &value).await?;
        }

        if old.diameter != new.diameter {
            let value = match new.diameter {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "diameter", &value).await?;
        }

        if old.state != new.state {
            self.add_tree_prop(new.id, "state", &new.state).await?;
        }

        if old.thumbnail_id != new.thumbnail_id {
            let value = match new.thumbnail_id {
                Some(value) => value.to_string(),
                None => "".to_string(),
            };

            self.add_tree_prop(new.id, "thumbnail_id", &value).await?;
        }

        Ok(())
    }

    fn tree_from_row(
        row: &async_sqlite::rusqlite::Row,
    ) -> std::result::Result<TreeRecord, async_sqlite::rusqlite::Error> {
        Ok(TreeRecord {
            id: row.get(0)?,
            osm_id: row.get(1)?,
            lat: row.get(2)?,
            lon: row.get(3)?,
            species: row.get(4)?,
            notes: row.get(5)?,
            height: row.get(6)?,
            circumference: row.get(7)?,
            diameter: row.get(8)?,
            state: row.get(9)?,
            added_at: row.get(10)?,
            updated_at: row.get(11)?,
            added_by: row.get(12)?,
            thumbnail_id: row.get(13)?,
            year: row.get(14)?,
            address: row.get(15)?,
        })
    }

    fn osm_tree_from_row(
        row: &async_sqlite::rusqlite::Row,
    ) -> std::result::Result<OsmTreeRecord, async_sqlite::rusqlite::Error> {
        Ok(OsmTreeRecord {
            id: row.get(0)?,
            lat: row.get(1)?,
            lon: row.get(2)?,
            genus: row.get(3)?,
            species: row.get(4)?,
            species_wikidata: row.get(5)?,
            height: row.get(6)?,
            circumference: row.get(7)?,
            diameter_crown: row.get(8)?,
        })
    }

    fn user_from_row(
        row: &async_sqlite::rusqlite::Row,
    ) -> std::result::Result<UserRecord, async_sqlite::rusqlite::Error> {
        Ok(UserRecord {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            picture: row.get(3)?,
        })
    }

    /**
     * Custom case-insensitive collation for SQLite.
     *
     * Unlike the LOWER() function, this supports Unicode.
     */
    fn case_insensitive_collation(a: &str, b: &str) -> Ordering {
        a.to_lowercase().cmp(&b.to_lowercase())
    }
}

impl Locatable for SqliteDatabase {
    fn create(_locator: &Locator) -> Result<Self> {
        futures::executor::block_on(Self::new())
    }
}

#[async_trait]
impl DatabaseInterface for SqliteDatabase {
    async fn add_tree(&self, tree: &TreeRecord) -> Result<()> {
        let tree = tree.clone();

        self.pool.conn(move |conn| {
            match conn.execute("INSERT INTO trees (id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", (tree.id, tree.osm_id, tree.lat, tree.lon, tree.species, tree.notes, tree.height, tree.circumference, tree.diameter, tree.state, tree.added_at, tree.updated_at, tree.added_by, tree.thumbnail_id)) {
                Ok(_) => (),

                Err(e) => {
                    error!("Error adding tree to the database: {}", e);
                    return Err(e);
                },
            };

            debug!("Tree {} added to the database.", tree.id);
            Ok(())
        }).await?;

        Ok(())
    }

    async fn update_tree(&self, tree: &TreeRecord) -> Result<()> {
        let tree = tree.clone();

        let old = match self.get_tree(tree.id).await? {
            Some(value) => value,

            None => {
                error!("Tree {} not found in the database.", tree.id);
                return Err(Error::DatabaseQuery);
            }
        };

        let new = tree.clone();

        self.pool.conn(move |conn| {
            match conn.execute("UPDATE trees SET osm_id = ?, lat = ?, lon = ?, species = ?, notes = ?, height = ?, circumference = ?, diameter = ?, state = ?, updated_at = ?, year = ?, address = ? WHERE id = ?", (tree.osm_id, tree.lat, tree.lon, tree.species, tree.notes, tree.height, tree.circumference, tree.diameter, tree.state, tree.updated_at, tree.year, tree.address, tree.id)) {
                Ok(_) => (),

                Err(e) => {
                    error!("Error updating a tree in the database: {}", e);
                    return Err(e);
                },
            };

            debug!("Tree {} updated.", tree.id);
            Ok(())
        }).await?;

        self.log_tree_changes(&old, &new).await?;

        Ok(())
    }

    async fn move_tree(&self, id: u64, lat: f64, lon: f64) -> Result<()> {
        let updated_at = get_timestamp();

        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "UPDATE trees set lat = ?, lon = ?, updated_at = ? WHERE id = ?",
                    (lat, lon, updated_at, id),
                ) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error updating a tree in the database: {}", e);
                        return Err(e);
                    }
                };

                debug!("Tree {} moved.", id);
                Ok(())
            })
            .await?;

        Ok(())
    }

    /**
     * Read information on a single tree.
     */
    async fn get_tree(&self, id: u64) -> Result<Option<TreeRecord>> {
        let tree = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE id = ? LIMIT 1") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([id]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            if let Some(row) = rows.next()? {
                return Ok(Some(Self::tree_from_row(row)?));
            }

            Ok(None)
        }).await?;

        Ok(tree)
    }

    /**
     * Read information on a single tree.
     */
    async fn get_tree_by_osm_id(&self, osm_id: u64) -> Result<Option<TreeRecord>> {
        let tree = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE osm_id = ? LIMIT 1") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([osm_id]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            if let Some(row) = rows.next()? {
                return Ok(Some(Self::tree_from_row(row)?));
            }

            Ok(None)
        }).await?;

        Ok(tree)
    }

    /**
     * Read all trees from the database.
     *
     * https://docs.rs/rusqlite/0.30.0/rusqlite/index.html
     */
    async fn get_trees(&self, bounds: Bounds) -> Result<Vec<TreeRecord>> {
        let trees = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE lat <= ? AND lat >= ? AND lon <= ? AND lon >= ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([bounds.n, bounds.s, bounds.e, bounds.w]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut trees: Vec<TreeRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                trees.push(Self::tree_from_row(row)?);
            }

            Ok(trees)
        }).await?;

        Ok(trees)
    }

    async fn get_trees_by_ids(&self, ids: &[u64]) -> Result<Vec<TreeRecord>> {
        let mut trees: Vec<TreeRecord> = Vec::new();

        for id in ids {
            let tree = self.get_tree(*id).await?;

            if let Some(tree) = tree {
                trees.push(tree);
            }
        }

        Ok(trees)
    }

    async fn get_new_trees(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let trees = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees ORDER BY added_at DESC LIMIT ?, ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([skip, count]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut trees: Vec<TreeRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                trees.push(Self::tree_from_row(row)?);
            }

            Ok(trees)
        }).await?;

        Ok(trees)
    }

    async fn get_updated_trees(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let trees = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE added_at <> updated_at ORDER BY updated_at DESC LIMIT ?, ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([skip, count]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut trees: Vec<TreeRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                trees.push(Self::tree_from_row(row)?);
            }

            Ok(trees)
        }).await?;

        Ok(trees)
    }

    /**
     * Count all trees that still exist.
     */
    async fn count_trees(&self) -> Result<u64> {
        let count: u64 = self
            .pool
            .conn(move |conn| {
                let mut stmt =
                    match conn.prepare("SELECT COUNT(1) FROM trees WHERE state <> 'gone'") {
                        Ok(value) => value,

                        Err(e) => {
                            error!("Error preparing SQL statement: {}", e);
                            return Err(e);
                        }
                    };

                let mut rows = match stmt.query([]) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error executing SQL statement: {}", e);
                        return Err(e);
                    }
                };

                match rows.next()? {
                    Some(row) => row.get(0),
                    None => Ok(0),
                }
            })
            .await?;

        Ok(count)
    }

    /**
     * Find the closest tree to the given coordinates.
     */
    async fn find_closest_trees(
        &self,
        lat: f64,
        lon: f64,
        distance: f64,
    ) -> Result<Vec<TreeRecord>> {
        let delta = distance / 111_111.0; // meters per degree

        let bounds = Bounds {
            n: lat + delta,
            s: lat - delta,
            e: lon + delta,
            w: lon - delta,
        };

        self.get_trees(bounds).await
    }

    async fn get_last_tree_by_user(&self, user_id: u64) -> Result<Option<TreeRecord>> {
        let tree = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE added_by = ? ORDER BY id DESC") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([user_id]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            if let Some(row) = rows.next()? {
                return Ok(Some(Self::tree_from_row(row)?));
            }

            Ok(None)
        }).await?;

        Ok(tree)
    }

    /**
     * Record a change in tree properties.
     *
     * Returns new property id.
     */
    async fn add_tree_prop(&self, tree_id: u64, name: &str, value: &str) -> Result<u64> {
        let id = get_unique_id()?;
        let added_at = crate::utils::get_timestamp();
        let name = name.to_string();
        let value = value.to_string();

        self.pool.conn(move |conn| {
            match conn.execute("INSERT INTO trees_props (id, tree_id, added_at, name, value) VALUES (?, ?, ?, ?, ?)", (id, tree_id, added_at, name, value)) {
                Ok(_) => debug!("Property {} added to tree {}.", id, tree_id),

                Err(e) => {
                    error!("Error adding property to tree: {}", e);
                    return Err(e);
                }
            }

            Ok(conn.last_insert_rowid())
        }).await?;

        Ok(id)
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserRecord>> {
        let email = email.to_string();

        let user = self
            .pool
            .conn(move |conn| {
                let mut stmt = match conn
                    .prepare("SELECT id, email, name, picture FROM users WHERE email = ?")
                {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error preparing SQL statement: {}", e);
                        return Err(e);
                    }
                };

                let mut rows = stmt.query([email])?;

                if let Some(row) = rows.next()? {
                    return Ok(Some(Self::user_from_row(row)?));
                }

                Ok(None)
            })
            .await?;

        Ok(user)
    }

    async fn add_user(&self, user: &UserRecord) -> Result<()> {
        let id = user.id;
        let email = user.email.clone();
        let name = user.name.clone();
        let picture = user.picture.clone();

        self.pool
            .conn(move |conn| {
                conn.execute(
                    "INSERT INTO users (id, email, name, picture) VALUES (?, ?, ?, ?)",
                    (id, email, name, picture),
                )?;
                debug!("User {} added to the database.", id);
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn get_user(&self, id: u64) -> Result<Option<UserRecord>> {
        let user = self
            .pool
            .conn(move |conn| {
                let mut stmt =
                    match conn.prepare("SELECT id, email, name, picture FROM users WHERE id = ?") {
                        Ok(value) => value,

                        Err(e) => {
                            error!("Error preparing SQL statement: {}", e);
                            return Err(e);
                        }
                    };

                let mut rows = stmt.query([id])?;

                if let Some(row) = rows.next()? {
                    return Ok(Some(Self::user_from_row(row)?));
                }

                Ok(None)
            })
            .await?;

        Ok(user)
    }

    async fn get_users(&self, ids: &[u64]) -> Result<Vec<UserRecord>> {
        let mut users: Vec<UserRecord> = Vec::new();

        for id in ids {
            let user = self.get_user(*id).await?;

            if let Some(user) = user {
                users.push(user);
            }
        }

        Ok(users)
    }

    async fn add_queue_message(&self, msg: &QueueMessage) -> Result<()> {
        let id = msg.id;
        let added_at = msg.added_at;
        let available_at = msg.available_at;
        let payload = msg.payload.clone();
        let attempts = msg.attempts;

        self.pool
            .conn(move |conn| {
                conn.execute(
                    "INSERT INTO queue_messages (id, added_at, available_at, payload, attempts) VALUES (?, ?, ?, ?, ?)",
                    (id, added_at, available_at, payload, attempts),
                )?;

                Ok(())
            })
            .await?;

        debug!("Queue message {} added to the database.", id);

        Ok(())
    }

    async fn pick_queue_message(&self) -> Result<Option<QueueMessage>> {
        let now = get_timestamp();

        let message = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, added_at, available_at, payload, attempts FROM queue_messages WHERE available_at <= ? AND attempts < 10 ORDER BY id LIMIT 1") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = stmt.query([now])?;

            if let Some(row) = rows.next()? {
                return Ok(Some(QueueMessage::from_sqlite(row)?));
            }

            Ok(None)
        }).await?;

        Ok(message)
    }

    async fn delay_queue_message(&self, id: u64, available_at: u64) -> Result<()> {
        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "UPDATE queue_messages SET available_at = ?, attempts = attempts + 1 WHERE id = ?",
                    (available_at, id),
                ) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error updating a queue message: {}", e);
                        return Err(e);
                    }
                };

                debug!("Queue message {} updated.", id);
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn delete_queue_message(&self, id: u64) -> Result<()> {
        self.pool
            .conn(move |conn| {
                match conn.execute("DELETE FROM queue_messages WHERE id = ?", [id]) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error deleting a queue message: {}", e);
                        return Err(e);
                    }
                };

                debug!("Queue message {} deleted.", id);
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn add_file(&self, file: &FileRecord) -> Result<()> {
        let id = file.id;
        let added_at = file.added_at;
        let added_by = file.added_by;
        let tree_id = file.tree_id;
        let small_id = file.small_id;
        let large_id = file.large_id;

        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "INSERT INTO files (id, tree_id, added_at, added_by, small_id, large_id) VALUES (?, ?, ?, ?, ?, ?)",
                    (id, tree_id, added_at, added_by, small_id, large_id),
                ) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error adding file to the database: {}", e);
                        return Err(e);
                    },
                }

                Ok(())
            })
            .await?;

        debug!("File {} added to the database.", id);

        Ok(())
    }

    async fn get_file(&self, id: u64) -> Result<Option<FileRecord>> {
        let file = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, tree_id, added_at, added_by, small_id, large_id FROM files WHERE id = ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = stmt.query([id])?;

            if let Some(row) = rows.next()? {
                return Ok(Some(FileRecord {
                    id: row.get(0)?,
                    tree_id: row.get(1)?,
                    added_at: row.get(2)?,
                    added_by: row.get(3)?,
                    small_id: row.get(4)?,
                    large_id: row.get(5)?,
                }));
            }

            Ok(None)
        }).await?;

        Ok(file)
    }

    async fn find_files_by_tree(&self, tree_id: u64) -> Result<Vec<FileRecord>> {
        let files = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, tree_id, added_at, added_by, small_id, large_id FROM files WHERE tree_id = ? AND small_id <> 0 AND large_id <> 0 ORDER BY id DESC") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = stmt.query([tree_id])?;
            let mut files: Vec<FileRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                files.push(FileRecord {
                    id: row.get(0)?,
                    tree_id: row.get(1)?,
                    added_at: row.get(2)?,
                    added_by: row.get(3)?,
                    small_id: row.get(4)?,
                    large_id: row.get(5)?,
                });
            }

            Ok(files)
        }).await?;

        Ok(files)
    }

    async fn update_file(&self, file: &FileRecord) -> Result<()> {
        let id = file.id;
        let small_id = file.small_id;
        let large_id = file.large_id;

        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "UPDATE files SET small_id = ?, large_id = ? WHERE id = ?",
                    (small_id, large_id, id),
                ) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error updating a file in the database: {}", e);
                        return Err(e);
                    }
                };

                debug!("File {} updated.", id);
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update_tree_thumbnail(&self, tree_id: u64, file_id: u64) -> Result<()> {
        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "UPDATE trees SET thumbnail_id = ? WHERE id = ?",
                    (file_id, tree_id),
                ) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error updating a tree thumbnail: {}", e);
                        return Err(e);
                    }
                };

                debug!("Thumbnail for tree {} updated.", tree_id);
                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn add_comment(&self, comment: &CommentRecord) -> Result<()> {
        let tmp = comment.clone();

        self.pool.conn(move |conn| {
            match conn.execute("INSERT INTO comments (id, tree_id, added_at, added_by, message) VALUES (?, ?, ?, ?, ?)", (tmp.id, tmp.tree_id, tmp.added_at, tmp.added_by, tmp.message)) {
                Ok(_) => (),

                Err(e) => {
                    error!("Error adding comment to the database: {}", e);
                    return Err(e);
                },
            };

            debug!("Comment {} added to the database.", tmp.id);
            Ok(())
        }).await?;

        Ok(())
    }

    async fn find_comments_by_tree(&self, tree_id: u64) -> Result<Vec<CommentRecord>> {
        let comments = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, tree_id, added_at, added_by, message FROM comments WHERE tree_id = ? ORDER BY added_at DESC") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = stmt.query([tree_id])?;
            let mut comments: Vec<CommentRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                comments.push(CommentRecord {
                    id: row.get(0)?,
                    tree_id: row.get(1)?,
                    added_at: row.get(2)?,
                    added_by: row.get(3)?,
                    message: row.get(4)?,
                });
            }

            Ok(comments)
        }).await?;

        Ok(comments)
    }

    async fn find_recent_comments(&self, count: u64) -> Result<Vec<CommentRecord>> {
        let comments = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, tree_id, added_at, added_by, message FROM comments ORDER BY added_at DESC LIMIT ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = stmt.query([count])?;
            let mut comments: Vec<CommentRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                comments.push(CommentRecord {
                    id: row.get(0)?,
                    tree_id: row.get(1)?,
                    added_at: row.get(2)?,
                    added_by: row.get(3)?,
                    message: row.get(4)?,
                });
            }

            Ok(comments)
        }).await?;

        Ok(comments)
    }

    async fn find_species(&self, query: &str) -> Result<Vec<SpeciesRecord>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let species = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT name, local, keywords, wikidata_id FROM species WHERE name LIKE ?1 OR local LIKE ?1 OR keywords LIKE ?1 ORDER BY name LIMIT 10") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([pattern]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut species: Vec<SpeciesRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                species.push(SpeciesRecord {
                    name: row.get(0)?,
                    local: row.get(1)?,
                    keywords: row.get(2)?,
                    wikidata_id: row.get(3)?,
                });
            }

            Ok(species)
        }).await?;

        Ok(species)
    }

    async fn get_osm_tree(&self, id: u64) -> Result<Option<OsmTreeRecord>> {
        let tree = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, lat, lon, genus, species, species_wikidata, height, circumference, diameter_crown FROM osm_trees WHERE id = ? LIMIT 1") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([id]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            if let Some(row) = rows.next()? {
                return Ok(Some(Self::osm_tree_from_row(row)?));
            }

            Ok(None)
        }).await?;

        Ok(tree)
    }

    async fn add_osm_tree(&self, tree: &OsmTreeRecord) -> Result<()> {
        let tmp = tree.clone();

        self.pool.conn(move |conn| {
            match conn.execute("DELETE FROM osm_trees WHERE id = ?", [tmp.id]) {
                Ok(_) => (),

                Err(e) => {
                    error!("Error deleting existing tree: {}", e);
                    return Err(e);
                },
            };

            match conn.execute("INSERT INTO osm_trees (id, lat, lon, genus, species, species_wikidata, height, circumference, diameter_crown) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", (tmp.id, tmp.lat, tmp.lon, tmp.genus, tmp.species, tmp.species_wikidata, tmp.height, tmp.circumference, tmp.diameter_crown)) {
                Ok(_) => (),

                Err(e) => {
                    error!("Error adding comment to the database: {}", e);
                    return Err(e);
                },
            };

            debug!("OSM Tree {} added to the database.", tmp.id);

            Ok(())
        }).await?;

        Ok(())
    }

    async fn find_osm_trees(&self) -> Result<Vec<OsmTreeRecord>> {
        let trees = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, lat, lon, genus, species, species_wikidata, height, circumference, diameter_crown FROM osm_trees") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut trees: Vec<OsmTreeRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                trees.push(Self::osm_tree_from_row(row)?);
            }

            Ok(trees)
        }).await?;

        Ok(trees)
    }

    /**
     * Find most recent species added by a specific user.
     */
    async fn find_recent_species(&self, user_id: u64) -> Result<Vec<String>> {
        let since = get_timestamp() - SUGGEST_WINDOW;

        let items = self.pool.conn(move |conn| {
            conn.create_collation("case_insensitive", Self::case_insensitive_collation)?;

            let mut stmt = match conn.prepare("SELECT species, COUNT(1) AS use_count FROM trees WHERE added_by = ? AND added_at >= ? AND LOWER(species) <> 'unknown' GROUP BY species COLLATE case_insensitive ORDER BY use_count DESC LIMIT 10") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([user_id, since]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut values: Vec<String> = Vec::new();

            while let Some(row) = rows.next()? {
                values.push(row.get(0)?);
            }

            Ok(values)
        }).await?;

        Ok(items)
    }

    async fn like_tree(&self, tree_id: u64, user_id: u64) -> Result<()> {
        let updated_at = crate::utils::get_timestamp();

        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "REPLACE INTO likes (tree_id, user_id, state, updated_at) VALUES (?, ?, 1, ?)",
                    (tree_id, user_id, updated_at),
                ) {
                    Ok(_) => debug!("Tree {} liked by user {}.", tree_id, user_id),

                    Err(e) => {
                        error!("Error liking a tree: {}", e);
                        return Err(e);
                    }
                }

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn unlike_tree(&self, tree_id: u64, user_id: u64) -> Result<()> {
        let updated_at = crate::utils::get_timestamp();

        self.pool
            .conn(move |conn| {
                match conn.execute(
                    "REPLACE INTO likes (tree_id, user_id, state, updated_at) VALUES (?, ?, 0, ?)",
                    (tree_id, user_id, updated_at),
                ) {
                    Ok(_) => debug!("Tree {} liked by user {}.", tree_id, user_id),

                    Err(e) => {
                        error!("Error liking a tree: {}", e);
                        return Err(e);
                    }
                }

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn find_trees_with_no_address(&self) -> Result<Vec<TreeRecord>> {
        let trees = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE address IS NULL ORDER BY added_at DESC") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut trees: Vec<TreeRecord> = Vec::new();

            while let Some(row) = rows.next()? {
                trees.push(Self::tree_from_row(row)?);
            }

            Ok(trees)
        }).await?;

        Ok(trees)
    }
}

impl Clone for SqliteDatabase {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use std::env;

    async fn setup() -> SqliteDatabase {
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let db = SqliteDatabase::new()
            .await
            .expect("Error creating database.");

        db.execute(include_str!("./fixtures/init.sql").to_string())
            .await
            .expect("Error installing fixtures.");

        db
    }

    #[tokio::test]
    async fn test_get_trees() -> Result<()> {
        let db = setup().await;
        db.execute(include_str!("./fixtures/trees.sql").to_string())
            .await
            .unwrap();

        let bounds = Bounds {
            n: 90.0,
            e: 180.0,
            s: -90.0,
            w: -180.0,
        };

        let trees = match db.get_trees(bounds).await {
            Ok(value) => value,

            Err(e) => {
                panic!("Error reading trees from the database: {}", e);
            }
        };

        assert_eq!(trees.len(), 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_add_tree() {
        let db = setup().await;

        db.add_tree(&TreeRecord {
            id: 123,
            osm_id: Some(234),
            lat: 56.65,
            lon: 28.48,
            species: "Quercus".to_string(),
            notes: Some("Big Oak".to_string()),
            height: Some(12.0),
            circumference: Some(1.0),
            diameter: Some(2.3),
            state: "healthy".to_string(),
            added_at: 12345,
            updated_at: 23456,
            added_by: 7,
            thumbnail_id: Some(8),
            year: None,
            address: None,
        })
        .await
        .expect("Error adding tree");

        let tree = db
            .get_tree(123)
            .await
            .expect("Error reading a tree that was just added")
            .expect("Tree not found.");

        assert_eq!(tree.id, 123, "wrong id");
        assert_eq!(tree.osm_id, Some(234), "wrong osm_id");
        assert_eq!(tree.lat, 56.65, "wrong lat");
        assert_eq!(tree.lon, 28.48, "wrong lon");
        assert_eq!(tree.species, "Quercus", "wrong species");
        assert_eq!(tree.notes, Some("Big Oak".to_string()), "wrong notes");
        assert_eq!(tree.height, Some(12.0), "wrong height");
        assert_eq!(tree.circumference, Some(1.0), "wrong circumference");
        assert_eq!(tree.diameter, Some(2.3), "wrong diameter");
        assert_eq!(tree.state, "healthy", "wrong state");
        assert_eq!(tree.added_at, 12345, "wrong added_at");
        assert_eq!(tree.updated_at, 23456, "wrong updated_at");
        assert_eq!(tree.added_by, 7, "wrong added_by");
        assert_eq!(tree.thumbnail_id, Some(8), "wrong thumbnail_id");
    }

    #[tokio::test]
    async fn test_add_queue_message() {
        let db = setup().await;
        let now = get_timestamp();

        let msg = QueueMessage {
            id: 123,
            added_at: now,
            available_at: now,
            payload: "it works".to_string(),
            attempts: 0,
        };

        db.add_queue_message(&msg)
            .await
            .expect("Error adding message.");

        let pick = db
            .pick_queue_message()
            .await
            .expect("Error picking message.");
        assert_eq!(pick.unwrap().id, 123);
    }

    #[tokio::test]
    async fn test_add_queue_message_not_available() {
        let db = setup().await;
        let now = get_timestamp();

        let msg = QueueMessage {
            id: 123,
            added_at: now,
            available_at: now + 10,
            payload: "it works".to_string(),
            attempts: 0,
        };

        db.add_queue_message(&msg)
            .await
            .expect("Error adding message.");

        let pick = db
            .pick_queue_message()
            .await
            .expect("Error picking message.");
        assert!(pick.is_none());
    }

    #[tokio::test]
    async fn test_delay_queue_message() {
        let db = setup().await;
        let now = get_timestamp();

        let msg = QueueMessage {
            id: 123,
            added_at: now,
            available_at: now,
            payload: "it works".to_string(),
            attempts: 0,
        };

        db.add_queue_message(&msg)
            .await
            .expect("Error adding message.");
        db.delay_queue_message(123, now + 10)
            .await
            .expect("Error delaying message.");

        let pick = db
            .pick_queue_message()
            .await
            .expect("Error picking message.");
        assert!(pick.is_none());
    }

    #[tokio::test]
    async fn test_pending_files_ignored() {
        let db = setup().await;

        db.add_file(&FileRecord {
            id: 1,
            added_at: 0,
            added_by: 0,
            tree_id: 2,
            small_id: 3,
            large_id: 0,
        })
        .await
        .expect("Error adding file.");

        db.add_file(&FileRecord {
            id: 2,
            added_at: 0,
            added_by: 0,
            tree_id: 2,
            small_id: 3,
            large_id: 5,
        })
        .await
        .expect("Error adding file.");

        let files = db
            .find_files_by_tree(2)
            .await
            .expect("Error finding files.");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].id, 2);
    }

    #[tokio::test]
    async fn test_comments() {
        let db = setup().await;
        let now = get_timestamp();

        let tree_id: u64 = 2;

        let comment = CommentRecord {
            id: 1,
            tree_id,
            added_at: now,
            added_by: 3,
            message: "it works".to_string(),
        };

        db.add_comment(&comment)
            .await
            .expect("Error adding comment.");

        let comments = db
            .find_comments_by_tree(tree_id)
            .await
            .expect("Error finding comments.");

        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].id, 1);
        assert_eq!(comments[0].tree_id, tree_id);
    }

    #[tokio::test]
    async fn test_find_species() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/species.sql").to_string())
            .await
            .expect("Error adding species.");

        let species = db
            .find_species(" oak ")
            .await
            .expect("Error finding species.");

        assert_eq!(species.len(), 1, "Could not find species for oak.");
        assert_eq!("Quercus robur", species[0].name);
    }

    #[tokio::test]
    async fn test_find_recent_species() {
        let db = setup().await;

        let now = get_timestamp();
        let user_id: u64 = 12345;

        for species_id in 0..10 {
            for _ in 0..species_id {
                db.add_tree(&TreeRecord {
                    id: get_unique_id().expect("Error generating tree id."),
                    osm_id: None,
                    lat: 0.0,
                    lon: 0.0,
                    species: format!("Species nr.{}", species_id + 1),
                    notes: None,
                    height: None,
                    circumference: None,
                    diameter: None,
                    state: "healthy".to_string(),
                    added_at: now,
                    added_by: user_id,
                    updated_at: now,
                    thumbnail_id: None,
                    year: None,
                    address: None,
                })
                .await
                .expect("Error adding tree.");
            }
        }

        let recent = db
            .find_recent_species(user_id)
            .await
            .expect("Error reading recent species.");
        assert_eq!(recent.len(), 9);
        assert_eq!(recent[0], "Species nr.10");
        assert_eq!(recent[1], "Species nr.9");
    }

    #[tokio::test]
    async fn test_find_recent_species_single() {
        let db = setup().await;

        let now = get_timestamp();
        let user_id: u64 = 12345;

        db.add_tree(&TreeRecord {
            id: get_unique_id().expect("Error generating tree id."),
            osm_id: None,
            lat: 0.0,
            lon: 0.0,
            species: "Some tree".to_string(),
            notes: None,
            height: None,
            circumference: None,
            diameter: None,
            state: "healthy".to_string(),
            added_at: now,
            added_by: user_id,
            updated_at: now,
            thumbnail_id: None,
            year: None,
            address: None,
        })
        .await
        .expect("Error adding tree");

        let recent = db
            .find_recent_species(user_id)
            .await
            .expect("Error reading recent species.");

        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0], "Some tree");
    }

    #[tokio::test]
    async fn test_find_recent_species_old() {
        let db = setup().await;

        let now = get_timestamp();
        let user_id: u64 = 12345;

        db.add_tree(&TreeRecord {
            id: get_unique_id().expect("Error generating tree id."),
            osm_id: None,
            lat: 0.0,
            lon: 0.0,
            species: "Some tree".to_string(),
            notes: None,
            height: None,
            circumference: None,
            diameter: None,
            state: "healthy".to_string(),
            added_at: now - 86400 * 7, // one week ago
            added_by: user_id,
            updated_at: now,
            thumbnail_id: None,
            year: None,
            address: None,
        })
        .await
        .expect("Error adding tree");

        let recent = db
            .find_recent_species(user_id)
            .await
            .expect("Error reading recent species.");

        assert_eq!(recent.len(), 0);
    }

    #[tokio::test]
    async fn test_suggest_case_insensitive() {
        let db = setup().await;

        let now = get_timestamp();
        let user_id = 1;

        let names = vec!["клён", "Клён", "Unknown", "КЛЁН"];

        for name in names {
            db.add_tree(&TreeRecord {
                id: get_unique_id().expect("Error generating tree id."),
                osm_id: None,
                lat: 0.0,
                lon: 0.0,
                species: name.to_string(),
                notes: None,
                height: None,
                circumference: None,
                diameter: None,
                state: "healthy".to_string(),
                added_at: now,
                added_by: user_id,
                updated_at: now,
                thumbnail_id: None,
                year: None,
                address: None,
            })
            .await
            .expect("Error adding tree.");
        }

        let recent = db
            .find_recent_species(user_id)
            .await
            .expect("Error reading recent species.");

        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0], "клён");
    }

    #[tokio::test]
    async fn test_count() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/test_count.sql").to_string())
            .await
            .expect("Error setting up data.");

        let count = db.count_trees().await.expect("Error counting trees.");
        assert_eq!(count, 2); // 1 dead + 1 healthy - 1 gone
    }
}
