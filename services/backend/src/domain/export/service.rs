use super::models::ExportFile;
use crate::infra::storage::BackupBucket;
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

pub struct ExportService {
    backups: Arc<BackupBucket>,
}

impl ExportService {
    pub async fn list_recent(&self, limit: usize) -> Result<Vec<ExportFile>> {
        let files = self.backups.list_files("export/").await?;

        // The `export/` prefix plus the `.sqlite.gz` suffix are the guardrails
        // that prevent full, non-anonymized backups from being exposed.
        let mut files: Vec<_> = files
            .into_iter()
            .filter(|file| file.path.ends_with(".sqlite.gz"))
            .collect();

        // Filenames are ISO dates (`YYYY-MM-DD`), so lexicographic ordering is
        // chronological; this depends on the export naming scheme.
        files.sort_by(|a, b| b.path.cmp(&a.path));
        files.truncate(limit);

        let mut res = Vec::new();

        for file in files {
            let url = self.backups.create_read_url(&file.path).await?;
            let name = file
                .path
                .strip_prefix("export/")
                .unwrap_or(file.path.as_str())
                .to_string();

            res.push(ExportFile {
                name,
                url,
                size: file.size,
            });
        }

        Ok(res)
    }
}

impl Injectable for ExportService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            backups: ctx.backups(),
        })
    }
}
