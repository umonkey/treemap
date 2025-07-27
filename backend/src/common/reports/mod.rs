//! Contains the code that converts input raw data to actual reports.

use crate::types::*;
use std::collections::HashMap;

pub fn format_species_report(items: Vec<(String, u64)>) -> Vec<SpeciesStatsResponse> {
    let mut buckets: HashMap<String, SpeciesStatsResponse> = HashMap::new();

    for (name, count) in items {
        let parts: Vec<&str> = name.splitn(2, ' ').collect();

        let genus = match parts[0] {
            "" => "Unknown".to_string(),
            genus => genus.to_string(),
        };

        let entry = buckets
            .entry(genus.clone())
            .or_insert(SpeciesStatsResponse {
                name: genus,
                count: 0,
                subspecies: vec![],
            });

        entry.count += count;

        if parts.len() > 1 {
            entry.subspecies.push(SpeciesReportItem { name, count });
        }
    }

    let mut report: Vec<SpeciesStatsResponse> = buckets.into_values().collect();
    report.sort_by(|a, b| b.count.cmp(&a.count));

    report
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
}
