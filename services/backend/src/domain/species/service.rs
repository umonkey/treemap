use super::models::Species;
use super::report::{calculate_simpson_index, format_species_report};
use super::schemas::{CreateSpeciesRequest, DiversityReport, SpeciesStats, UpdateSpeciesRequest};
use super::utils::{get_genus_name, get_species_name};
use crate::domain::species::SpeciesRepository;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SpeciesService {
    repo: Arc<SpeciesRepository>,
}

impl SpeciesService {
    pub async fn search(&self, query: &str) -> Result<Vec<Species>> {
        self.repo.search(query).await
    }

    pub async fn suggest(&self, user_id: u64) -> Result<Vec<String>> {
        self.repo.suggest(user_id).await
    }

    pub async fn list(&self) -> Result<Vec<Species>> {
        self.repo.all().await
    }

    pub async fn get_counts(&self) -> Result<HashMap<String, u64>> {
        self.repo.get_counts().await
    }

    pub async fn get(&self, name: &str) -> Result<Species> {
        self.repo.get(name).await?.ok_or(Error::SpeciesNotFound)
    }

    pub async fn create(&self, req: CreateSpeciesRequest) -> Result<Species> {
        let name = req.name.trim().to_string();
        let local = req.local.trim().to_string();

        if name.is_empty() {
            return Err(Error::BadRequestMessage(
                "Species name cannot be empty.".to_string(),
            ));
        }

        if local.is_empty() {
            return Err(Error::BadRequestMessage(
                "Species local name cannot be empty.".to_string(),
            ));
        }

        let wikidata_id = req.wikidata_id.trim().to_string();
        validate_wikidata_id(&wikidata_id)?;

        let species = Species {
            name: name.clone(),
            local,
            keywords: normalize_keywords(&req.keywords),
            wikidata_id,
        };

        self.repo.add(&species).await?;

        info!("Species {name} created by {}.", req.user_id);

        Ok(species)
    }

    pub async fn update(&self, name: &str, req: UpdateSpeciesRequest) -> Result<Species> {
        let name = name.trim().to_string();
        let local = req.local.trim().to_string();

        if name.is_empty() {
            return Err(Error::BadRequestMessage(
                "Species name cannot be empty.".to_string(),
            ));
        }

        if local.is_empty() {
            return Err(Error::BadRequestMessage(
                "Species local name cannot be empty.".to_string(),
            ));
        }

        // Ensure the record exists before updating.
        self.repo.get(&name).await?.ok_or(Error::SpeciesNotFound)?;

        let wikidata_id = req.wikidata_id.trim().to_string();
        validate_wikidata_id(&wikidata_id)?;

        let species = Species {
            name: name.clone(),
            local,
            keywords: normalize_keywords(&req.keywords),
            wikidata_id,
        };

        self.repo.update(&species).await?;

        info!("Species {name} updated by {}: {species:?}", req.user_id);

        Ok(species)
    }

    pub async fn get_stats(&self) -> Result<Vec<SpeciesStats>> {
        let items = self.get_raw_stats().await?;
        let report = format_species_report(items);
        Ok(report)
    }

    pub async fn get_diversity_index(&self) -> Result<DiversityReport> {
        let species = self.get_raw_stats().await?;

        let counts: Vec<u64> = species
            .clone()
            .into_iter()
            .map(|(_, count)| count)
            .collect();

        let total_count = counts.clone().iter().sum();

        let index = calculate_simpson_index(&counts);
        let excess_species = self.get_warning_species(&species, total_count);
        let excess_genera = self.get_warning_genera(&species, total_count);

        Ok(DiversityReport {
            index,
            excess_species,
            excess_genera,
        })
    }

    async fn get_raw_stats(&self) -> Result<Vec<(String, u64)>> {
        self.repo.get_raw_stats().await
    }

    // Find species above 10% threshold.
    fn get_warning_species(
        &self,
        species: &[(String, u64)],
        total_count: u64,
    ) -> Vec<(String, u64)> {
        let mut counts: HashMap<String, u64> = HashMap::new();

        for (name, count) in species {
            if let Some(value) = get_species_name(name) {
                counts
                    .entry(value)
                    .and_modify(|val| *val += count)
                    .or_insert(*count);
            }
        }

        let mut warn: Vec<(String, u64)> = Vec::new();

        for (species, count) in &counts {
            let percent = count * 100 / total_count;

            if percent > 10 {
                warn.push((species.to_string(), percent));
            }
        }

        debug!(
            "There are {} species with ≥{} trees (10%).",
            warn.len(),
            total_count / 10
        );

        warn
    }

    // Find genus above 20% threshold.
    fn get_warning_genera(
        &self,
        species: &[(String, u64)],
        total_count: u64,
    ) -> Vec<(String, u64)> {
        let mut counts: HashMap<String, u64> = HashMap::new();

        for (name, count) in species {
            if let Some(value) = get_genus_name(name) {
                counts
                    .entry(value)
                    .and_modify(|val| *val += count)
                    .or_insert(*count);
            }
        }

        let mut warn: Vec<(String, u64)> = Vec::new();

        for (species, count) in &counts {
            let percent = count * 100 / total_count;

            if percent > 20 {
                warn.push((species.to_string(), percent));
            }
        }

        debug!(
            "There are {} genera with ≥{} trees (20%).",
            warn.len(),
            total_count / 5
        );

        warn
    }
}

/// Trim each keyword, drop empties, and join them back with `;`.
fn normalize_keywords(raw: &str) -> String {
    raw.split(';')
        .map(|keyword| keyword.trim())
        .filter(|keyword| !keyword.is_empty())
        .collect::<Vec<&str>>()
        .join(";")
}

/// Accept an empty id, or a Wikidata id of the form `Q` followed by digits.
fn validate_wikidata_id(id: &str) -> Result<()> {
    if id.is_empty() {
        return Ok(());
    }

    let valid = id
        .strip_prefix('Q')
        .map(|digits| !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()))
        .unwrap_or(false);

    if !valid {
        return Err(Error::BadRequestMessage(format!(
            "Invalid Wikidata id: {id}."
        )));
    }

    Ok(())
}

impl Injectable for SpeciesService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<SpeciesRepository>()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::database::Database;
    use crate::services::AppState;
    use crate::services::ContextExt;
    use crate::utils::get_timestamp;
    use log::debug;

    async fn setup() -> (SpeciesService, Arc<Database>) {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        let db = state.database();

        let service = state
            .build::<SpeciesService>()
            .expect("Error creating SpeciesService");

        (service, db)
    }

    #[tokio::test]
    async fn test_search() {
        let (service, db) = setup().await;

        db.execute_batch(include_str!("../../infra/database/fixtures/species.sql"))
            .await
            .expect("Error adding species.");

        let species = service
            .search(" oak ")
            .await
            .expect("Error finding species.");

        assert_eq!(species.len(), 1, "Could not find species for oak.");
        assert_eq!("Quercus robur", species[0].name);
    }

    #[tokio::test]
    async fn test_suggest() {
        let (service, db) = setup().await;

        db.execute_sql("DELETE FROM trees", &[])
            .await
            .expect("Error clearing trees.");

        let now = get_timestamp();

        db.execute_sql(format!(
            "INSERT INTO trees (id, lat, lon, species, added_at, updated_at, updated_by, added_by) VALUES (1, 40.1, 44.1, 'Birch', {}, {}, 1, 1)",
            now, now
        ).as_str(), &[])
            .await
            .expect("Error adding tree.");

        let suggestion = service.suggest(1).await.expect("Error getting suggestion.");

        assert_eq!(suggestion.len(), 1);
        assert_eq!(suggestion[0], "Birch");
    }

    #[tokio::test]
    async fn test_get_stats() {
        let (service, db) = setup().await;

        db.execute_batch(include_str!(
            "../../infra/database/fixtures/test_species_stats.sql"
        ))
        .await
        .expect("Error adding species.");

        let res = service.get_stats().await.expect("Error getting report.");

        assert_eq!(res.len(), 1);
        assert_eq!("Quercus", res[0].name);
        assert_eq!(2, res[0].count);
    }

    #[tokio::test]
    async fn test_create_and_get() {
        let (service, _) = setup().await;

        let created = service
            .create(CreateSpeciesRequest {
                name: "  Quercus robur  ".to_string(),
                local: " Pedunculate oak ".to_string(),
                keywords: " дуб ; oak ; ".to_string(),
                wikidata_id: " Q165145 ".to_string(),
                user_id: 1,
            })
            .await
            .expect("Error creating species.");

        assert_eq!(created.name, "Quercus robur");
        assert_eq!(created.keywords, "дуб;oak");

        let found = service
            .get("Quercus robur")
            .await
            .expect("Error getting species.");

        assert_eq!(found.local, "Pedunculate oak");
    }

    #[tokio::test]
    async fn test_update() {
        let (service, _) = setup().await;

        service
            .create(CreateSpeciesRequest {
                name: "Quercus robur".to_string(),
                local: "Pedunculate oak".to_string(),
                keywords: "дуб;oak".to_string(),
                wikidata_id: "Q165145".to_string(),
                user_id: 1,
            })
            .await
            .expect("Error creating species.");

        let updated = service
            .update(
                "Quercus robur",
                UpdateSpeciesRequest {
                    local: "Common oak".to_string(),
                    keywords: "дуб;oak;common".to_string(),
                    wikidata_id: "".to_string(),
                    user_id: 2,
                },
            )
            .await
            .expect("Error updating species.");

        assert_eq!(updated.local, "Common oak");
        assert_eq!(updated.wikidata_id, "");
    }

    #[tokio::test]
    async fn test_get_missing() {
        let (service, _) = setup().await;

        let res = service.get("Nonexistent").await;

        assert!(matches!(res, Err(Error::SpeciesNotFound)));
    }
}
