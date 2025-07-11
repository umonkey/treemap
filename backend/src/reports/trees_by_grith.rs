//! This class reports distribution of trees by trunk grith.
//! The size is reported in cm, rounder by 10.
//! Ignores stumps.

use crate::services::*;
use crate::types::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TreesByGrithReport {
    pub value: u64,
    pub count: usize,
}

pub struct TreesByGrithReporter {}

impl TreesByGrithReporter {
    pub fn report(&self, trees: &Vec<TreeRecord>) -> Result<Vec<TreesByGrithReport>> {
        let map = self.aggregate(trees);
        let mut res = self.convert(map);

        res.sort_by(|a, b| a.value.cmp(&b.value));

        Ok(res)
    }

    fn aggregate(&self, trees: &Vec<TreeRecord>) -> HashMap<u64, usize> {
        let mut map: HashMap<u64, usize> = HashMap::new();

        for tree in trees {
            if tree.state == "stump" {
                continue;
            }

            let size = tree.circumference.unwrap_or(0.0);

            if size <= 0.0 {
                continue;
            }

            let cm = ((size * 10.0).round() as u64) * 10;
            let count = map.entry(cm).or_insert(0);
            *count += 1;
        }

        map
    }

    fn convert(&self, map: HashMap<u64, usize>) -> Vec<TreesByGrithReport> {
        let mut result: Vec<TreesByGrithReport> = Vec::new();

        for (value, count) in map {
            result.push(TreesByGrithReport { value, count });
        }

        result
    }
}

impl Locatable for TreesByGrithReporter {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesByGrithReporter {
        TreesByGrithReporter {}
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
            circumference: Some(1.2),
            ..Default::default()
        });

        trees.push(TreeRecord {
            circumference: Some(1.55),
            ..Default::default()
        });

        trees.push(TreeRecord {
            circumference: Some(1.57),
            ..Default::default()
        });

        let reporter = setup();
        let report = reporter.report(&trees).unwrap();
        assert!(!report.is_empty());
        assert_eq!(report.len(), 2);

        assert_eq!(report[0].value, 120);
        assert_eq!(report[0].count, 1);

        assert_eq!(report[1].value, 160);
        assert_eq!(report[1].count, 2);
    }
}
