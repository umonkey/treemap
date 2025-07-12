//! This class reports total area of the crowns.
//! Ignores stumps and dead trees.

use crate::services::*;
use crate::types::*;

pub struct TreesAreaReporter {}

impl TreesAreaReporter {
    pub fn report(&self, trees: &Vec<TreeRecord>) -> Result<u64> {
        let mut total: f64 = 0.0;

        for tree in trees {
            total += self.get_tree_shadow(tree);
        }

        Ok(total.round() as u64)
    }

    fn get_tree_shadow(&self, tree: &TreeRecord) -> f64 {
        if tree.state == "stump" || tree.state == "dead" {
            return 0.0;
        }

        if let Some(size) = tree.diameter {
            if size > 0.0 {
                return (size / 2.0).powi(2) * std::f64::consts::PI; // Area of a circle with radius = diameter / 2
            }
        }

        0.0
    }
}

impl Locatable for TreesAreaReporter {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TreesAreaReporter {
        TreesAreaReporter {}
    }

    #[test]
    fn test_empty() {
        let reporter = setup();
        let res = reporter.report(&vec![]).unwrap();
        assert_eq!(res, 0);
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
        let res = reporter.report(&trees).unwrap();

        assert_eq!(res, 5);
    }
}
