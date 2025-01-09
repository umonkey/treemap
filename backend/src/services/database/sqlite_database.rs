//! Database client implementation for SQLite.
//!
//! Uses async-sqlite.
//!
//! Probably nee to split the core database code into a separate class,
//! and the queue implementation should also be separate, with its own database.
//!
//! @docs https://docs.rs/async-sqlite/latest/async_sqlite/

use crate::common::database::queries::*;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_sqlite_path, get_timestamp};
use async_sqlite::{JournalMode, Pool, PoolBuilder};
use async_trait::async_trait;
use log::{debug, error, info};
use rusqlite::params_from_iter;
use rusqlite::types::Value;
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
                error!("Error connecting to the database: {:?}", e);
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

    /**
     * Custom case-insensitive collation for SQLite.
     *
     * Unlike the LOWER() function, this supports Unicode.
     */
    fn case_insensitive_collation(a: &str, b: &str) -> Ordering {
        a.to_lowercase().cmp(&b.to_lowercase())
    }

    fn pack_record(row: &rusqlite::Row, fields: &Vec<&str>) -> rusqlite::Result<Attributes> {
        let mut props = Vec::<(String, Value)>::new();

        for field in fields {
            let value = row.get(*field)?;
            props.push((field.to_string(), value));
        }

        Ok(Attributes::from(&props))
    }
}

impl Locatable for SqliteDatabase {
    fn create(_locator: &Locator) -> Result<Self> {
        futures::executor::block_on(Self::new())
    }
}

#[async_trait]
impl DatabaseInterface for SqliteDatabase {
    async fn get_record(&self, query: SelectQuery) -> Result<Option<Attributes>> {
        let query = SelectQuery {
            limit: Some(1),
            offset: None,
            ..query
        };

        let record = self
            .pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                let mut stmt = match conn.prepare(sql.as_str()) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error preparing SQL statement: {}", e);
                        return Err(e);
                    }
                };

                let mut rows = stmt.query(params_from_iter(params.iter())).map_err(|e| {
                    error!("Error executing SQL statement: {}", e);
                    e
                })?;

                if let Some(row) = rows.next()? {
                    let fields = row.as_ref().column_names();
                    let rec = Self::pack_record(row, &fields)?;
                    Ok(Some(rec))
                } else {
                    Ok(None)
                }
            })
            .await?;

        Ok(record)
    }

    async fn get_records(&self, query: SelectQuery) -> Result<Vec<Attributes>> {
        let record = self
            .pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                let mut stmt = match conn.prepare(sql.as_str()) {
                    Ok(value) => value,

                    Err(e) => {
                        error!("Error preparing SQL statement: {}", e);
                        return Err(e);
                    }
                };

                let mut rows = stmt.query(params_from_iter(params.iter())).map_err(|e| {
                    error!("Error executing SQL statement: {}", e);
                    e
                })?;

                let mut records = Vec::new();

                while let Some(row) = rows.next()? {
                    let fields = row.as_ref().column_names();
                    let rec = Self::pack_record(row, &fields)?;
                    records.push(rec);
                }

                Ok(records)
            })
            .await?;

        Ok(record)
    }

    async fn add_record(&self, query: InsertQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error adding a record to the database: {}", e);
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn update(&self, query: UpdateQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error updating database: {}", e);
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

        Ok(())
    }

    async fn delete(&self, query: DeleteQuery) -> Result<()> {
        self.pool
            .conn(move |conn| {
                let (sql, params) = query.build();

                match conn.execute(sql.as_str(), params_from_iter(params)) {
                    Ok(_) => (),

                    Err(e) => {
                        error!("Error deleting record: {}", e);
                        return Err(e);
                    }
                };

                Ok(())
            })
            .await?;

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
                    return Ok(Some(UserRecord::from_sqlite_row(row)?));
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
                    return Ok(Some(UserRecord::from_sqlite_row(row)?));
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
            let mut stmt = match conn.prepare("SELECT id, tree_id, added_at, added_by, deleted_at, deleted_by, small_id, large_id FROM files WHERE id = ?") {
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
                    deleted_at: row.get(4)?,
                    deleted_by: row.get(5)?,
                    small_id: row.get(6)?,
                    large_id: row.get(7)?,
                }));
            }

            Ok(None)
        }).await?;

        Ok(file)
    }

    async fn find_files_by_tree(&self, tree_id: u64) -> Result<Vec<FileRecord>> {
        let files = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, tree_id, added_at, added_by, deleted_at, deleted_by, small_id, large_id FROM files WHERE tree_id = ? AND small_id <> 0 AND large_id <> 0 ORDER BY id DESC") {
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
                    deleted_at: row.get(4)?,
                    deleted_by: row.get(5)?,
                    small_id: row.get(6)?,
                    large_id: row.get(7)?,
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

    async fn get_species_stats(&self) -> Result<Vec<(String, u64)>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT species, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' GROUP BY species ORDER BY cnt DESC, LOWER(species)") {
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

            let mut res = Vec::new();

            while let Some(row) = rows.next()? {
                let species: String = row.get(0)?;
                let count: u64 = row.get(1)?;
                res.push((species, count));
            }

            Ok(res)
        }).await?;

        Ok(res)
    }

    async fn get_species_mismatch(&self, count: u64, skip: u64) -> Result<Vec<TreeRecord>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT id, osm_id, lat, lon, species, notes, height, circumference, diameter, state, added_at, updated_at, added_by, thumbnail_id, year, address FROM trees WHERE state <> 'gone' AND species <> 'Unknown species' AND species <> 'Unknown' AND species NOT IN (SELECT name FROM species) LIMIT ? OFFSET ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([count, skip]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut records = Vec::new();

            while let Some(row) = rows.next()? {
                let fields = row.as_ref().column_names();
                let rec = Self::pack_record(row, &fields)?;

                if let Ok(packed) = TreeRecord::from_attributes(&rec) {
                    records.push(packed);
                }
            }

            Ok(records)
        }).await?;

        Ok(res)
    }

    async fn get_top_streets(&self, count: u64) -> Result<Vec<(String, u64)>> {
        let res = self.pool.conn(move |conn| {
            let mut stmt = match conn.prepare("SELECT address, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND address IS NOT NULL GROUP BY LOWER(address) ORDER BY cnt DESC, LOWER(address) LIMIT ?") {
                Ok(value) => value,

                Err(e) => {
                    error!("Error preparing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut rows = match stmt.query([count]) {
                Ok(value) => value,

                Err(e) => {
                    error!("Error executing SQL statement: {}", e);
                    return Err(e);
                },
            };

            let mut res = Vec::new();

            while let Some(row) = rows.next()? {
                let address: String = row.get(0)?;
                let count: u64 = row.get(1)?;
                res.push((address, count));
            }

            Ok(res)
        }).await?;

        Ok(res)
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
            tree_id: 2,
            small_id: 3,
            ..Default::default()
        })
        .await
        .expect("Error adding file.");

        db.add_file(&FileRecord {
            id: 2,
            tree_id: 2,
            small_id: 3,
            large_id: 5,
            ..Default::default()
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
    async fn test_count() {
        let db = setup().await;

        db.execute(include_str!("./fixtures/test_count.sql").to_string())
            .await
            .expect("Error setting up data.");

        let count = db.count_trees().await.expect("Error counting trees.");
        assert_eq!(count, 2); // 1 dead + 1 healthy - 1 gone
    }
}
