//! This class reports distribution of trees by species.
//! Ignores stumps.

use crate::services::*;
use crate::types::*;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct TreesBySpeciesReport {
    pub species: String,
    pub count: usize,
    pub height: f64,
    pub diameter: f64,
    pub grith: f64,
}

pub struct TreesBySpeciesReporter {}

impl TreesBySpeciesReporter {
    pub fn report(&self, trees: &Vec<TreeRecord>) -> Result<Vec<TreesBySpeciesReport>> {
        let species = self.get_species(trees);
        let mut res = self.get_reports(&species, trees);

        res.sort_by(|a, b| a.species.cmp(&b.species));

        Ok(res)
    }

    fn get_species(&self, trees: &Vec<TreeRecord>) -> Vec<String> {
        let mut species: Vec<String> = Vec::new();

        for tree in trees {
            if tree.state == "stump" {
                continue;
            }

            if tree.species.is_empty() {
                continue;
            }

            if !species.contains(&tree.species) {
                species.push(tree.species.clone());
            }
        }

        species
    }

    fn get_reports(
        &self,
        species: &Vec<String>,
        trees: &[TreeRecord],
    ) -> Vec<TreesBySpeciesReport> {
        let mut reports: Vec<TreesBySpeciesReport> = Vec::new();

        for spec in species {
            reports.push(self.get_report(spec, trees));
        }

        reports
    }

    fn get_report(&self, species: &str, trees: &[TreeRecord]) -> TreesBySpeciesReport {
        let mut count: usize = 0;
        let mut heights: Vec<f64> = Vec::new();
        let mut diameters: Vec<f64> = Vec::new();
        let mut griths: Vec<f64> = Vec::new();

        let trees = self.filter_by_species(species, trees);

        for tree in trees {
            count += 1;

            if let Some(height) = tree.height {
                if height > 0.0 {
                    heights.push(height);
                }
            }

            if let Some(diameter) = tree.diameter {
                if diameter > 0.0 {
                    diameters.push(diameter);
                }
            }

            if let Some(grith) = tree.circumference {
                if grith > 0.0 {
                    griths.push(grith);
                }
            }
        }

        if count == 0 {
            return TreesBySpeciesReport {
                species: species.to_string(),
                ..Default::default()
            };
        }

        TreesBySpeciesReport {
            species: species.to_string(),
            count,
            height: Self::avg(&heights),
            diameter: Self::avg(&diameters),
            grith: Self::avg(&griths),
        }
    }

    fn filter_by_species(&self, species: &str, trees: &[TreeRecord]) -> Vec<TreeRecord> {
        if species.is_empty() {
            return vec![];
        }

        trees
            .iter()
            .filter(|tree| tree.species == species && tree.state != "stump")
            .cloned()
            .collect()
    }

    fn avg(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }

        let sum: f64 = values.iter().sum();
        (sum / values.len() as f64 * 100.0).round() / 100.0
    }
}

impl Locatable for TreesBySpeciesReporter {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesBySpeciesReporter {
        TreesBySpeciesReporter {}
    }

    #[test]
    fn test_empty() {
        let reporter = setup();
        let report = reporter.report(&vec![]).unwrap();
        assert!(report.is_empty());
    }

    #[test]
    fn test_success() {
        let mut trees = vec![];

        trees.push(TreeRecord {
            species: "Ulmus".to_string(),
            diameter: Some(1.2),
            ..Default::default()
        });

        trees.push(TreeRecord {
            species: "Ulmus".to_string(),
            height: Some(1.4),
            ..Default::default()
        });

        trees.push(TreeRecord {
            species: "Ulmus".to_string(),
            diameter: Some(1.6),
            ..Default::default()
        });

        let reporter = setup();
        let report = reporter.report(&trees).unwrap();
        assert!(!report.is_empty());
        assert_eq!(report.len(), 1);

        assert_eq!(report[0].species, "Ulmus".to_string());
        assert_eq!(report[0].count, 3);
        assert_eq!(report[0].height, 1.4);
        assert_eq!(report[0].diameter, 1.4);
        assert_eq!(report[0].grith, 0.0);
    }
}
