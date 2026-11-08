//! Implements the local file system storage driver.
//! This is very limited and should only be used for unit testing.

use super::base::{CompletedPart, StorageDriver, StorageFile};
use crate::infra::config::Config;
use crate::types::*;
use async_trait::async_trait;
use log::{debug, error, info};
use tokio::fs;

pub struct LocalStorageDriver {
    folder: String,
}

impl LocalStorageDriver {
    pub fn new(config: &Config) -> Self {
        Self {
            folder: config.file_folder.clone(),
        }
    }
}

fn modified_at(metadata: &std::fs::Metadata) -> Option<u64> {
    metadata
        .modified()
        .ok()
        .and_then(|value| value.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|value| value.as_secs())
}

#[async_trait]
impl StorageDriver for LocalStorageDriver {
    async fn write_file(&self, bucket: &str, path: &str, data: &[u8], _public: bool) -> Result<()> {
        let bucket_folder = format!("{}/{}", self.folder, bucket);
        let file_path = format!("{}/{}", bucket_folder, path);

        let parent = match std::path::Path::new(&file_path).parent() {
            Some(value) => value,

            None => {
                error!("Error resolving folder for {}: no parent.", file_path);
                return Err(Error::FileUpload);
            }
        };

        match fs::create_dir_all(parent).await {
            Ok(()) => (),

            Err(e) => {
                error!("Error creating folder {}: {:?}", parent.display(), e);
                return Err(Error::FileUpload);
            }
        };

        match fs::write(file_path, data).await {
            Ok(()) => {
                info!("File {}/{} written, {} bytes.", bucket, path, data.len());
                Ok(())
            }

            Err(e) => {
                error!("Error writing file: {:?}", e);
                Err(Error::FileUpload)
            }
        }
    }

    async fn read_file(&self, bucket: &str, path: &str) -> Result<Vec<u8>> {
        let file_path = format!("{}/{}/{}", self.folder, bucket, path);

        match fs::read(file_path).await {
            Ok(value) => {
                debug!("File {}/{} read, {} bytes.", bucket, path, value.len());
                Ok(value)
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("File {}/{} not found.", bucket, path);
                Err(Error::FileNotFound)
            }

            Err(e) => {
                error!("Error reading file {}/{}: {:?}", bucket, path, e);
                Err(Error::FileDownload)
            }
        }
    }

    async fn create_upload_url(&self, _bucket: &str, _path: &str) -> Result<String> {
        Err(Error::FileUpload)
    }

    async fn create_read_url(&self, _bucket: &str, _path: &str) -> Result<String> {
        Err(Error::FileDownload)
    }

    async fn exists(&self, bucket: &str, path: &str) -> Result<bool> {
        let file_path = format!("{}/{}/{}", self.folder, bucket, path);
        Ok(fs::metadata(file_path).await.is_ok())
    }

    async fn list_files(&self, bucket: &str, prefix: &str) -> Result<Vec<StorageFile>> {
        let bucket_folder = std::path::PathBuf::from(format!("{}/{}", self.folder, bucket));
        let search_path = bucket_folder.join(prefix);
        let mut files = Vec::new();

        if let Ok(metadata) = tokio::fs::metadata(&search_path).await {
            if search_path.is_file() {
                if let Ok(rel) = search_path.strip_prefix(&bucket_folder) {
                    if let Some(rel_str) = rel.to_str() {
                        files.push(StorageFile {
                            path: rel_str.replace('\\', "/"),
                            size: metadata.len(),
                            etag: None,
                            last_modified_at: modified_at(&metadata),
                        });
                    }
                }
            } else {
                let mut stack = vec![search_path];
                while let Some(dir) = stack.pop() {
                    let mut entries = match fs::read_dir(&dir).await {
                        Ok(entries) => entries,
                        Err(_) => continue,
                    };
                    while let Some(entry) = entries
                        .next_entry()
                        .await
                        .map_err(|_| Error::FileDownload)?
                    {
                        let path = entry.path();
                        if path.is_dir() {
                            stack.push(path);
                        } else if path.is_file() {
                            if let Ok(metadata) = fs::metadata(&path).await {
                                if let Ok(rel) = path.strip_prefix(&bucket_folder) {
                                    if let Some(rel_str) = rel.to_str() {
                                        files.push(StorageFile {
                                            path: rel_str.replace('\\', "/"),
                                            size: metadata.len(),
                                            etag: None,
                                            last_modified_at: modified_at(&metadata),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(files)
    }

    async fn start_multipart_upload(&self, _bucket: &str, _path: &str) -> Result<String> {
        Err(Error::FileUpload)
    }

    async fn create_upload_part_url(
        &self,
        _bucket: &str,
        _path: &str,
        _upload_id: &str,
        _part_number: i32,
    ) -> Result<String> {
        Err(Error::FileUpload)
    }

    async fn complete_multipart_upload(
        &self,
        _bucket: &str,
        _path: &str,
        _upload_id: &str,
        _parts: Vec<CompletedPart>,
    ) -> Result<()> {
        Err(Error::FileUpload)
    }

    async fn copy_file(&self, bucket: &str, from: &str, to: &str) -> Result<()> {
        let from_path = format!("{}/{}/{}", self.folder, bucket, from);
        let to_path = format!("{}/{}/{}", self.folder, bucket, to);

        let parent = match std::path::Path::new(&to_path).parent() {
            Some(value) => value,

            None => {
                error!("Error resolving folder for {}: no parent.", to_path);
                return Err(Error::FileUpload);
            }
        };

        if let Err(e) = fs::create_dir_all(parent).await {
            error!("Error creating folder {}: {:?}", parent.display(), e);
            return Err(Error::FileUpload);
        }

        match fs::copy(&from_path, &to_path).await {
            Ok(_) => {
                debug!("File {}/{} copied to {}.", bucket, from, to);
                Ok(())
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
                debug!("File {}/{} not found, cannot copy.", bucket, from);
                Err(Error::FileNotFound)
            }

            Err(e) => {
                error!("Error copying file {}/{}: {:?}", bucket, from, e);
                Err(Error::FileUpload)
            }
        }
    }

    async fn delete_file(&self, bucket: &str, path: &str) -> Result<()> {
        let file_path = format!("{}/{}/{}", self.folder, bucket, path);
        match fs::remove_file(&file_path).await {
            Ok(()) => {
                debug!("File {}/{} deleted.", bucket, path);
                Ok(())
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => {
                error!("Error deleting file {}/{}: {:?}", bucket, path, e);
                Err(Error::FileUpload)
            }
        }
    }

    async fn delete_files(&self, bucket: &str, paths: &[String]) -> Result<()> {
        for path in paths {
            let file_path = format!("{}/{}/{}", self.folder, bucket, path);
            match fs::remove_file(&file_path).await {
                Ok(()) => {
                    debug!("File {}/{} deleted.", bucket, path);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    error!("Error deleting file {}/{}: {:?}", bucket, path, e);
                    return Err(Error::FileUpload);
                }
            }
        }

        Ok(())
    }
}
