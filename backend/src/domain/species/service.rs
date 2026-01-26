use super::models::Species;
use super::report::{calculate_simpson_index, format_species_report};
use super::schemas::{DiversityReport, SpeciesStats};
use super::utils::{get_genus_name, get_species_name};
use crate::infra::database::Database;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;

pub struct SpeciesService {
    db: Arc<Database>,
}

impl SpeciesService {
    pub async fn search(&self, query: &str) -> Result<Vec<Species>> {
        self.db.find_species(query).await
    }

    pub async fn suggest(&self, user_id: u64) -> Result<Vec<String>> {
        self.db.find_recent_species(user_id).await
    }

    pub async fn get_stats(&self) -> Result<Vec<SpeciesStats>> {
        let items = self.db.get_species_stats().await?;
        let report = format_species_report(items);
        Ok(report)
    }

    pub async fn get_diversity_index(&self) -> Result<DiversityReport> {
        let species = self.db.get_species_stats().await?;

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

impl Locatable for SpeciesService {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
