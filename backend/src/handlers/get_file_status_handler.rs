use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetFileStatusHandler {
    files: Arc<FileRepository>,
}

impl GetFileStatusHandler {
    pub async fn handle(&self, id: u64) -> Result<FileStatusResponse> {
        match self.files.get(id).await? {
            Some(file) => Ok(FileStatusResponse::from_file(&file)),
            None => Err(Error::FileNotFound),
        }
    }
}

impl Locatable for GetFileStatusHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileRepository>()?,
        })
    }
}
