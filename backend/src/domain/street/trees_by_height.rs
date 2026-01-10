//! This class reports distribution of trees by height.
//! Ignores stumps.

use super::schemas::TreesByHeightReport;
use crate::domain::tree::Tree;
use crate::types::Result;
use std::collections::HashMap;

pub struct TreesByHeightReporter {}

impl TreesByHeightReporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report(&self, trees: &Vec<Tree>) -> Result<Vec<TreesByHeightReport>> {
        let map = self.aggregate(trees);
        let mut res = self.convert(map);

        res.sort_by(|a, b| a.value.cmp(&b.value));

        Ok(res)
    }

    fn aggregate(&self, trees: &Vec<Tree>) -> HashMap<u64, usize> {
        let mut map: HashMap<u64, usize> = HashMap::new();

        for tree in trees {
            if tree.state == "stump" {
                continue;
            }

            let height = tree.height.unwrap_or(0.0);

            if height <= 0.0 {
                continue;
            }

            let meters = height.round() as u64;
            let count = map.entry(meters).or_insert(0);
            *count += 1;
        }

        map
    }

    fn convert(&self, map: HashMap<u64, usize>) -> Vec<TreesByHeightReport> {
        let mut result: Vec<TreesByHeightReport> = Vec::new();

        for (value, count) in map {
            result.push(TreesByHeightReport { value, count });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesByHeightReporter {
        TreesByHeightReporter::new();
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

        trees.push(Tree {
            height: Some(1.2),
            ..Default::default()
        });

        trees.push(Tree {
            height: Some(1.4),
            ..Default::default()
        });

        trees.push(Tree {
            height: Some(1.6),
            ..Default::default()
        });

        let reporter = setup();
        let report = reporter.report(&trees).unwrap();
        assert!(!report.is_empty());
        assert_eq!(report.len(), 2);

        assert_eq!(report[0].value, 1);
        assert_eq!(report[0].count, 2);

        assert_eq!(report[1].value, 2);
        assert_eq!(report[1].count, 1);
    }
}
