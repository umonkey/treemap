//! This class reports total area of the crowns.
//! Ignores stumps and dead trees.

use crate::domain::tree::Tree;
use crate::services::*;
use crate::types::*;

pub struct TreesAreaReporter {}

impl TreesAreaReporter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn report(&self, trees: &Vec<Tree>) -> Result<(f64, f64)> {
        let mut total: f64 = 0.0;
        let mut count: u64 = 0;

        for tree in trees {
            let area = self.get_tree_shadow(tree);

            if area > 0.0 {
                total += area;
                count += 1;
            }
        }

        let average_area = if count > 0 {
            total / (count as f64)
        } else {
            0.0
        };

        Ok((total, average_area))
    }

    fn get_tree_shadow(&self, tree: &Tree) -> f64 {
        if !tree.is_existing() || tree.state == "dead" {
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
        assert_eq!(res, (0.0, 0.0));
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
        let res = reporter.report(&trees).unwrap();

        let expected_total = 1.49 * std::f64::consts::PI;
        assert!((res.0 - expected_total).abs() < 1e-10);
        assert!((res.1 - expected_total / 3.0).abs() < 1e-10);
    }
}
