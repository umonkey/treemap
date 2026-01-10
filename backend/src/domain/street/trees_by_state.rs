//! This class reports distribution of trees by state.

use super::schemas::TreesByStateReport;
use crate::domain::tree::Tree;
use crate::types::Result;
use std::collections::HashMap;

pub struct TreesByStateReporter {}

impl TreesByStateReporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report(&self, trees: &Vec<Tree>) -> Result<Vec<TreesByStateReport>> {
        let map = self.aggregate(trees);
        let mut res = self.convert(map);

        res.sort_by(|a, b| a.state.cmp(&b.state));

        Ok(res)
    }

    fn aggregate(&self, trees: &Vec<Tree>) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();

        for tree in trees {
            let state = tree.state.clone();

            if state == "gone" {
                continue;
            }

            let count = map.entry(state).or_insert(0);
            *count += 1;
        }

        map
    }

    fn convert(&self, map: HashMap<String, usize>) -> Vec<TreesByStateReport> {
        let mut result: Vec<TreesByStateReport> = Vec::new();

        for (state, count) in map {
            result.push(TreesByStateReport { state, count });
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesByStateReporter {
        TreesByStateReporter::new();
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
            state: "healthy".to_string(),
            ..Default::default()
        });

        trees.push(Tree {
            state: "sick".to_string(),
            ..Default::default()
        });

        trees.push(Tree {
            state: "healthy".to_string(),
            ..Default::default()
        });

        trees.push(Tree {
            state: "gone".to_string(),
            ..Default::default()
        });

        let reporter = setup();
        let report = reporter.report(&trees).unwrap();
        assert!(!report.is_empty());
        assert_eq!(report.len(), 2);

        assert_eq!(report[0].state, "healthy");
        assert_eq!(report[0].count, 2);

        assert_eq!(report[1].state, "sick");
        assert_eq!(report[1].count, 1);
    }
}
