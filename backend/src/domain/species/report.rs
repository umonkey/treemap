//! Contains the code that converts input raw data to actual reports.

use super::schemas::*;
use std::collections::HashMap;
use log::debug;

pub fn format_species_report(items: Vec<(String, u64)>) -> Vec<SpeciesStats> {
    let mut buckets: HashMap<String, SpeciesStats> = HashMap::new();

    for (name, count) in items {
        let parts: Vec<&str> = name.splitn(2, ' ').collect();

        let genus = match parts[0] {
            "" => "Unknown".to_string(),
            genus => genus.to_string(),
        };

        let entry = buckets.entry(genus.clone()).or_insert(SpeciesStats {
            name: genus,
            count: 0,
            subspecies: vec![],
        });

        entry.count += count;

        if parts.len() > 1 {
            entry.subspecies.push(SpeciesStatsItem { name, count });
        }
    }

    let mut report: Vec<SpeciesStats> = buckets.into_values().collect();
    report.sort_by(|a, b| b.count.cmp(&a.count));

    report
}

pub fn calculate_simpson_index(counts: &[u64]) -> f64 {
    let total_count: u64 = counts.iter().sum();

    let mut index: f64 = 0.0;

    for count in counts {
        let value = ((*count as f64) / (total_count as f64)).powf(2.0);
        index += value;
    }

    debug!("Calculated Simpson index for {} trees; total_count={total_count}, value={index}", counts.len());

    1.0 - index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genus() {
        let report = format_species_report(vec![("Quercus".to_string(), 10)]);

        assert_eq!(1, report.len());
        assert_eq!("Quercus", report[0].name);
        assert_eq!(10, report[0].count);
        assert_eq!(0, report[0].subspecies.len());
    }

    #[test]
    fn test_species() {
        let report = format_species_report(vec![("Quercus robur".to_string(), 10)]);

        assert_eq!(1, report.len());
        assert_eq!("Quercus", report[0].name);
        assert_eq!(10, report[0].count);
        assert_eq!(1, report[0].subspecies.len());

        assert_eq!("Quercus robur", report[0].subspecies[0].name);
        assert_eq!(10, report[0].subspecies[0].count);
    }

    #[test]
    fn test_grouping() {
        let report = format_species_report(vec![
            ("Quercus robur".to_string(), 10),
            ("Quercus rubra".to_string(), 20),
        ]);

        assert_eq!(1, report.len());
        assert_eq!("Quercus", report[0].name);
        assert_eq!(30, report[0].count);
        assert_eq!(2, report[0].subspecies.len());

        assert_eq!("Quercus robur", report[0].subspecies[0].name);
        assert_eq!(10, report[0].subspecies[0].count);

        assert_eq!("Quercus rubra", report[0].subspecies[1].name);
        assert_eq!(20, report[0].subspecies[1].count);
    }

    #[test]
    fn test_simpson_index() {
        let data = &[35, 35, 30, 7];
        let index = calculate_simpson_index(data);
        assert_eq!(0.7031181762599354, index);
    }
}
