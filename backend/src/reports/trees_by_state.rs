//! This class reports distribution of trees by state.

use crate::services::*;
use crate::types::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TreesByStateReport {
    pub state: String,
    pub count: usize,
}

pub struct TreesByStateReporter {}

impl TreesByStateReporter {
    pub fn report(&self, trees: &Vec<TreeRecord>) -> Result<Vec<TreesByStateReport>> {
        let map = self.aggregate(trees);
        let mut res = self.convert(map);

        res.sort_by(|a, b| a.state.cmp(&b.state));

        Ok(res)
    }

    fn aggregate(&self, trees: &Vec<TreeRecord>) -> HashMap<String, usize> {
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

impl Locatable for TreesByStateReporter {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesByStateReporter {
        TreesByStateReporter {}
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
            state: "healthy".to_string(),
            ..Default::default()
        });

        trees.push(TreeRecord {
            state: "sick".to_string(),
            ..Default::default()
        });

        trees.push(TreeRecord {
            state: "healthy".to_string(),
            ..Default::default()
        });

        trees.push(TreeRecord {
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
