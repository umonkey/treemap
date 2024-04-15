use log::{debug, info, error};
use tokio::fs;

use crate::Result;
use crate::types::AddFileRequest;
use crate::utils::{get_file_folder, get_unique_id};
use crate::errors::Error;

pub struct FileService {
    folder: String,
}

impl FileService {
    pub fn init() -> Result<Self> {
        let folder = get_file_folder();

        Ok(Self {
            folder,
        })
    }

    pub async fn add_file(&self, req: AddFileRequest) -> Result<u64> {
        let id = self.write_file(&req.file).await?;
        info!("User {} added file {} to tree {}", req.user_id, id, req.tree_id);
        Ok(id)
    }

    async fn write_file(&self, data: &Vec<u8>) -> Result<u64> {
        let id = get_unique_id()?;
        let file_path = format!("{}/{}", self.folder, id);

        debug!("Writing file to: {}", file_path);

        match fs::create_dir_all(&self.folder).await {
            Ok(()) => (),

            Err(e) => {
                error!("Error creating folder: {:?}", e);
                return Err(Error::FileUpload);
            },
        };

        match fs::write(file_path, data).await {
            Ok(()) => Ok(id),

            Err(e) => {
                error!("Error writing file: {:?}", e);
                Err(Error::FileUpload)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use std::env;
    use std::path::Path;
    use super::*;

    fn setup() -> Result<FileService> {
        env::set_var("FILE_FOLDER", "var/test-files");

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        FileService::init()
    }

    #[tokio::test]
    async fn test_add_file() {
        let service = setup().unwrap();

        let req = AddFileRequest {
            user_id: 1,
            tree_id: 2,
            file: b"it works".to_vec(),
        };

        let file_id = service.add_file(req).await.expect("Failed to store the file");
        let file_path = format!("var/test-files/{}", file_id);
        let path = Path::new(&file_path);

        assert!(path.exists());

        let data = std::fs::read_to_string(path).expect("Failed to read the file");
        assert!(data == "it works");

        std::fs::remove_file(path).expect("Failed to remove the file");
    }
}
