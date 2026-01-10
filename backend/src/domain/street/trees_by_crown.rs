//! This class reports distribution of trees by value.
//! Ignores stumps.

use super::schemas::TreesByCrownReport;
use crate::domain::tree::Tree;
use crate::types::Result;
use std::collections::HashMap;

pub struct TreesByCrownReporter {}

impl TreesByCrownReporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report(&self, trees: &Vec<Tree>) -> Result<Vec<TreesByCrownReport>> {
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

            let value = tree.diameter.unwrap_or(0.0);

            if value <= 0.0 {
                continue;
            }

            let meters = value.round() as u64;
            let count = map.entry(meters).or_insert(0);
            *count += 1;
        }

        map
    }

    fn convert(&self, map: HashMap<u64, usize>) -> Vec<TreesByCrownReport> {
        let mut result: Vec<TreesByCrownReport> = Vec::new();

        for (value, count) in map {
            result.push(TreesByCrownReport { value, count });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesByCrownReporter {
        TreesByCrownReporter {}
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
            diameter: Some(1.2),
            ..Default::default()
        });

        trees.push(Tree {
            diameter: Some(1.4),
            ..Default::default()
        });

        trees.push(Tree {
            diameter: Some(1.6),
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
