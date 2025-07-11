//! This class reports distribution of trees by size.

use crate::services::*;
use crate::types::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct TreesByCrownReport {
    pub size: u64,
    pub count: usize,
}

pub struct TreesByCrownReporter {}

impl TreesByCrownReporter {
    pub fn report(&self, trees: &Vec<TreeRecord>) -> Result<Vec<TreesByCrownReport>> {
        let map = self.aggregate(trees);
        let mut res = self.convert(map);

        res.sort_by(|a, b| a.size.cmp(&b.size));

        Ok(res)
    }

    fn aggregate(&self, trees: &Vec<TreeRecord>) -> HashMap<u64, usize> {
        let mut map: HashMap<u64, usize> = HashMap::new();

        for tree in trees {
            let size = tree.diameter.unwrap_or(0.0);

            if size <= 0.0 {
                continue;
            }

            let meters = size.round() as u64;
            let count = map.entry(meters).or_insert(0);
            *count += 1;
        }

        map
    }

    fn convert(&self, map: HashMap<u64, usize>) -> Vec<TreesByCrownReport> {
        let mut result: Vec<TreesByCrownReport> = Vec::new();

        for (size, count) in map {
            result.push(TreesByCrownReport { size, count });
        }

        result
    }
}

impl Locatable for TreesByCrownReporter {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {})
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

        trees.push(TreeRecord {
            diameter: Some(1.2),
            ..Default::default()
        });

        trees.push(TreeRecord {
            diameter: Some(1.4),
            ..Default::default()
        });

        trees.push(TreeRecord {
            diameter: Some(1.6),
            ..Default::default()
        });

        let reporter = setup();
        let report = reporter.report(&trees).unwrap();
        assert!(!report.is_empty());
        assert_eq!(report.len(), 2);

        assert_eq!(report[0].size, 1);
        assert_eq!(report[0].count, 2);

        assert_eq!(report[1].size, 2);
        assert_eq!(report[1].count, 1);
    }
}
