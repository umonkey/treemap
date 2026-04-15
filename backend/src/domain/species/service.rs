use super::models::Species;
use super::report::{calculate_simpson_index, format_species_report};
use super::schemas::{DiversityReport, SpeciesStats};
use super::utils::{get_genus_name, get_species_name};
use crate::infra::database::{Database, Value};
use crate::services::{Context, Injectable};
use crate::types::Result;
use crate::utils::get_timestamp;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;

const SUGGEST_WINDOW: u64 = 3600 * 24; // 24 hours

pub struct SpeciesService {
    db: Arc<Database>,
}

impl SpeciesService {
    pub async fn search(&self, query: &str) -> Result<Vec<Species>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let rows = self
            .db
            .fetch_sql(
                "SELECT name, local, keywords, wikidata_id FROM species WHERE name LIKE ?1 OR local LIKE ?1 OR keywords LIKE ?1 ORDER BY name LIMIT 10",
                &[Value::from(pattern)],
            )
            .await?;

        let mut species: Vec<Species> = Vec::new();

        for row in rows {
            species.push(Species {
                name: row.require_string("name")?,
                local: row.require_string("local")?,
                keywords: row.require_string("keywords")?,
                wikidata_id: row.require_string("wikidata_id")?,
            });
        }

        Ok(species)
    }

    pub async fn suggest(&self, user_id: u64) -> Result<Vec<String>> {
        let since = get_timestamp() - SUGGEST_WINDOW;

        let rows = self
            .db
            .fetch_sql(
                "SELECT species, COUNT(1) AS use_count FROM trees WHERE updated_by = ? AND updated_at >= ? AND LOWER(species) <> 'unknown' GROUP BY species ORDER BY use_count DESC LIMIT 10",
                &[Value::from(user_id), Value::from(since)],
            )
            .await?;

        let mut values: Vec<String> = Vec::new();

        for row in rows {
            values.push(row.require_string("species")?);
        }

        Ok(values)
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
        let rows = self
            .db
            .fetch_sql(
                "SELECT species, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND state <> 'stump' GROUP BY TRIM(LOWER(species)) ORDER BY cnt DESC, LOWER(species)",
                &[],
            )
            .await?;

        let mut res = Vec::new();

        for row in rows {
            let species = row.require_string("species")?;
            let count = row.require_u64("cnt")?;
            res.push((species, count));
        }

        Ok(res)
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

impl Injectable for SpeciesService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;

    async fn setup() -> SpeciesService {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new().await.expect("Error creating app state.");

        state
            .build::<SpeciesService>()
            .expect("Error creating SpeciesService")
    }

    #[tokio::test]
    async fn test_search() {
        let service = setup().await;

        service
            .db
            .execute_batch(include_str!("../../infra/database/fixtures/species.sql"))
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
        let service = setup().await;

        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .expect("Error clearing trees.");

        let now = get_timestamp();

        service
            .db
            .execute_sql(format!(
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
        let service = setup().await;

        service
            .db
            .execute_batch(include_str!(
                "../../infra/database/fixtures/test_species_stats.sql"
            ))
            .await
            .expect("Error adding species.");

        let res = service.get_stats().await.expect("Error getting report.");

        assert_eq!(res.len(), 1);
        assert_eq!("Quercus", res[0].name);
        assert_eq!(2, res[0].count);
    }
}
