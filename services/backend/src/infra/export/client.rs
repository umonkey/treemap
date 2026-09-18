//! Writes an anonymized copy of the database into a temporary SQLite file.
//!
//! The export uses its own libsql connection with `journal_mode = MEMORY` so
//! that no `-wal` sidecar is left behind, which would otherwise produce an
//! incomplete gzip archive.

use crate::domain::observation::Observation;
use crate::domain::tree::Tree;
use crate::domain::tree_image::TreeImage;
use crate::domain::water::WaterSource;
use crate::infra::database::{InsertQuery, Value};
use crate::types::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use libsql::{params_from_iter, Builder, Transaction};
use std::io::prelude::*;
use std::io::Read;
use tempfile::NamedTempFile;

const EXPORT_SCHEMA: &str = r#"
CREATE TABLE trees (
    id INTEGER PRIMARY KEY,
    osm_id INTEGER,
    added_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    lat REAL NOT NULL,
    lon REAL NOT NULL,
    species TEXT NOT NULL,
    state TEXT NOT NULL,
    height REAL,
    diameter REAL,
    circumference REAL
);

CREATE TABLE trees_images (
    id INTEGER PRIMARY KEY,
    tree_id INTEGER NOT NULL,
    added_at INTEGER NOT NULL,
    url TEXT NOT NULL
);

CREATE TABLE observations (
    id INTEGER PRIMARY KEY,
    tree_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    bark_damage INTEGER NOT NULL,
    dry_branches INTEGER NOT NULL,
    leaking INTEGER NOT NULL,
    root_damage INTEGER NOT NULL,
    open_roots INTEGER NOT NULL,
    topping INTEGER NOT NULL,
    fungal_bodies INTEGER NOT NULL,
    vfork INTEGER NOT NULL,
    cavities INTEGER NOT NULL,
    vines INTEGER NOT NULL,
    nests INTEGER NOT NULL,
    nesting_boxes INTEGER NOT NULL,
    bug_holes INTEGER NOT NULL,
    inclined INTEGER NOT NULL
);

CREATE TABLE water_source (
    id INTEGER PRIMARY KEY,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    status TEXT NOT NULL,
    lat REAL NOT NULL,
    lon REAL NOT NULL
);
"#;

pub struct ExportClient {
    file: NamedTempFile,
    tx: Transaction,
}

impl ExportClient {
    pub async fn new() -> Result<Self> {
        let file = NamedTempFile::new()
            .map_err(|e| Error::DatabaseQuery(format!("Error creating export file: {e}")))?;

        let path = file
            .path()
            .to_str()
            .ok_or_else(|| Error::DatabaseQuery("Invalid export file path".to_string()))?;

        let db = Builder::new_local(path).build().await?;
        let conn = db.connect()?;

        conn.execute_batch("PRAGMA journal_mode = MEMORY; PRAGMA synchronous = OFF;")
            .await?;

        conn.execute_batch(EXPORT_SCHEMA).await?;

        let tx = conn.transaction().await?;

        Ok(Self { file, tx })
    }

    pub async fn add_tree(&self, tree: &Tree) -> Result<()> {
        let query = InsertQuery::new("trees")
            .with_value("id", Value::from(tree.id as i64))
            .with_value("osm_id", Value::from(tree.osm_id))
            .with_value("added_at", Value::from(tree.added_at as i64))
            .with_value("updated_at", Value::from(tree.updated_at as i64))
            .with_value("lat", Value::from(tree.lat))
            .with_value("lon", Value::from(tree.lon))
            .with_value("species", Value::from(tree.species.clone()))
            .with_value("state", Value::from(tree.state.as_str()))
            .with_value("height", Value::from(tree.height))
            .with_value("diameter", Value::from(tree.diameter))
            .with_value("circumference", Value::from(tree.circumference));

        self.execute(query).await
    }

    pub async fn add_file(&self, file: &TreeImage, base_url: &str) -> Result<()> {
        let id = if file.source_id > 0 {
            file.source_id
        } else {
            file.large_id
        };

        let url = format!("{}{}.jpg", base_url, id);

        let query = InsertQuery::new("trees_images")
            .with_value("id", Value::from(file.id as i64))
            .with_value("tree_id", Value::from(file.tree_id as i64))
            .with_value("added_at", Value::from(file.added_at as i64))
            .with_value("url", Value::from(url));

        self.execute(query).await
    }

    pub async fn add_observation(&self, observation: &Observation) -> Result<()> {
        let query = InsertQuery::new("observations")
            .with_value("id", Value::from(observation.id as i64))
            .with_value("tree_id", Value::from(observation.tree_id as i64))
            .with_value("created_at", Value::from(observation.created_at as i64))
            .with_value("bark_damage", Value::from(observation.bark_damage))
            .with_value("dry_branches", Value::from(observation.dry_branches))
            .with_value("leaking", Value::from(observation.leaking))
            .with_value("root_damage", Value::from(observation.root_damage))
            .with_value("open_roots", Value::from(observation.open_roots))
            .with_value("topping", Value::from(observation.topping))
            .with_value("fungal_bodies", Value::from(observation.fungal_bodies))
            .with_value("vfork", Value::from(observation.vfork))
            .with_value("cavities", Value::from(observation.cavities))
            .with_value("vines", Value::from(observation.vines))
            .with_value("nests", Value::from(observation.nests))
            .with_value("nesting_boxes", Value::from(observation.nesting_boxes))
            .with_value("bug_holes", Value::from(observation.bug_holes))
            .with_value("inclined", Value::from(observation.inclined));

        self.execute(query).await
    }

    pub async fn add_water(&self, source: &WaterSource) -> Result<()> {
        let query = InsertQuery::new("water_source")
            .with_value("id", Value::from(source.id as i64))
            .with_value("created_at", Value::from(source.created_at as i64))
            .with_value("updated_at", Value::from(source.updated_at as i64))
            .with_value("status", Value::from(source.status.as_str()))
            .with_value("lat", Value::from(source.lat))
            .with_value("lon", Value::from(source.lon));

        self.execute(query).await
    }

    pub async fn finalize(self) -> Result<NamedTempFile> {
        let Self { file, tx } = self;

        tx.commit().await?;

        let mut source = std::fs::File::open(file.path())
            .map_err(|e| Error::DatabaseQuery(format!("Error opening export file: {e}")))?;

        let mut buffer = Vec::new();

        source
            .read_to_end(&mut buffer)
            .map_err(|e| Error::DatabaseQuery(format!("Error reading export file: {e}")))?;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

        encoder
            .write_all(&buffer)
            .map_err(|e| Error::DatabaseQuery(format!("Error compressing export: {e}")))?;

        let compressed = encoder
            .finish()
            .map_err(|e| Error::DatabaseQuery(format!("Error finishing export: {e}")))?;

        let mut output = NamedTempFile::new()
            .map_err(|e| Error::DatabaseQuery(format!("Error creating export file: {e}")))?;

        output
            .write_all(&compressed)
            .map_err(|e| Error::DatabaseQuery(format!("Error writing export file: {e}")))?;

        Ok(output)
    }

    async fn execute(&self, query: InsertQuery) -> Result<()> {
        let (sql, params) = query.build();

        self.tx.execute(&sql, params_from_iter(params)).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tree::Tree;

    #[tokio::test]
    async fn test_export_tree() -> Result<()> {
        let client = ExportClient::new().await?;

        client
            .add_tree(&Tree {
                id: 1,
                lat: 40.0,
                lon: 44.0,
                species: "Platanus".to_string(),
                ..Default::default()
            })
            .await?;

        let zipped = client.finalize().await?;

        let mut compressed = Vec::new();

        std::fs::File::open(zipped.path())
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?
            .read_to_end(&mut compressed)
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        let mut decompressed = Vec::new();

        flate2::read::GzDecoder::new(&compressed[..])
            .read_to_end(&mut decompressed)
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        let mut db_file = NamedTempFile::new().map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        db_file
            .write_all(&decompressed)
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        let path = db_file
            .path()
            .to_str()
            .ok_or_else(|| Error::DatabaseQuery("Invalid path".to_string()))?;

        let db = Builder::new_local(path)
            .build()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        let conn = db
            .connect()
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        let mut rows = conn
            .query(
                "SELECT species FROM trees WHERE id = 1",
                params_from_iter(std::iter::empty::<Value>()),
            )
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        let row = rows
            .next()
            .await
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?
            .ok_or_else(|| Error::DatabaseQuery("Row not found".to_string()))?;

        let species: String = row
            .get(0)
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        assert_eq!(species, "Platanus");

        Ok(())
    }
}
