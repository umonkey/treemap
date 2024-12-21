use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetFileStatusHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetFileStatusHandler {
    pub fn new(db: Arc<SqliteDatabase>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, id: u64) -> Result<FileStatusResponse> {
        match self.db.get_file(id).await? {
            Some(file) => Ok(FileStatusResponse::from_file(&file)),
            None => Err(Error::FileNotFound),
        }
    }
}

impl Locatable for GetFileStatusHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<SqliteDatabase>()?;
        Ok(Self::new(db))
    }
}
