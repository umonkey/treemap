/**
 * This structure represents a search query.
 *
 * We parse it to extract the words to search for, and flags to disable certain
 * search features.
 */
use crate::types::TreeRecord;

#[derive(Debug)]
pub struct SearchQuery {
    pub words: Vec<String>,
    pub nometrics: bool,
    pub noimages: bool,
    pub sick: bool,
    pub dead: bool,
    pub deformed: bool,
    pub healthy: bool,
}

impl SearchQuery {
    pub fn from_string(query: &str) -> SearchQuery {
        let mut words = Vec::new();
        let mut nometrics = false;
        let mut noimages = false;
        let mut sick = false;
        let mut dead = false;
        let mut deformed = false;
        let mut healthy = false;

        for word in query.split_whitespace() {
            if word.contains("nometric") {
                nometrics = true;
            } else if word.contains("noimage") || word.contains("nophoto") {
                noimages = true;
            } else if word.contains("sick") {
                sick = true;
            } else if word.contains("dead") {
                dead = true;
            } else if word.contains("deformed") {
                deformed = true;
            } else if word.contains("healthy") {
                healthy = true;
            } else {
                words.push(word.to_string().to_lowercase());
            }
        }

        SearchQuery {
            words,
            nometrics,
            noimages,
            sick,
            dead,
            deformed,
            healthy,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
            && !self.nometrics
            && !self.noimages
            && !self.sick
            && !self.dead
            && !self.deformed
            && !self.healthy
    }

    pub fn r#match(&self, tree: &TreeRecord) -> bool {
        if !self.match_text(tree) {
            return false;
        }

        if self.noimages && tree.thumbnail_id.is_some() {
            return false;
        }

        if self.nometrics
            && tree.height.is_some()
            && tree.circumference.is_some()
            && tree.diameter.is_some()
        {
            return false;
        }

        if self.sick && tree.state != "sick" {
            return false;
        }

        if self.dead && tree.state != "dead" {
            return false;
        }

        if self.deformed && tree.state != "deformed" {
            return false;
        }

        if self.healthy && tree.state != "healthy" {
            return false;
        }

        true
    }

    fn match_text(&self, tree: &TreeRecord) -> bool {
        if self.words.is_empty() {
            return true;
        }

        let text = Self::get_tree_text(tree);

        for word in &self.words {
            if !text.contains(word) {
                return false;
            }
        }

        true
    }

    fn get_tree_text(tree: &TreeRecord) -> String {
        let mut words: Vec<String> = Vec::new();
        words.push(tree.species.to_lowercase());

        if let Some(value) = &tree.notes {
            words.push(value.to_lowercase());
        }

        words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_tree() -> TreeRecord {
        TreeRecord {
            id: 0,
            osm_id: None,
            lat: 0.0,
            lon: 0.0,
            species: "Thuja plicata".to_string(),
            notes: None,
            height: None,
            circumference: None,
            diameter: None,
            state: "healthy".to_string(),
            added_at: 0,
            updated_at: 0,
            added_by: 0,
            thumbnail_id: None,
        }
    }

    #[test]
    fn test_query() {
        let query = SearchQuery::from_string("hello world");
        assert_eq!(query.words, vec!["hello", "world"]);
        assert_eq!(query.nometrics, false);
        assert_eq!(query.noimages, false);
    }

    #[test]
    fn test_nophoto() {
        let query = SearchQuery::from_string("hello world nophoto");
        assert_eq!(query.words, vec!["hello", "world"]);
        assert_eq!(query.noimages, true);
    }

    #[test]
    fn test_nometric() {
        let query = SearchQuery::from_string("hello world  nometrics");
        assert_eq!(query.words, vec!["hello", "world"]);
        assert_eq!(query.nometrics, true);
    }

    #[test]
    fn test_combined() {
        let query = SearchQuery::from_string("thuja  plicata  nometrics nophoto");
        assert_eq!(query.words, vec!["thuja", "plicata"]);
        assert_eq!(query.nometrics, true);
        assert_eq!(query.noimages, true);
    }

    #[test]
    fn test_empty() {
        let query = SearchQuery::from_string("");
        assert_eq!(query.words.len(), 0);
        assert_eq!(query.nometrics, false);
        assert_eq!(query.noimages, false);
        assert_eq!(query.is_empty(), true);
    }

    #[test]
    fn test_match_text() {
        let query = SearchQuery::from_string("thuja");

        let tree = TreeRecord {
            species: "Thuja plicata".to_string(),
            ..default_tree()
        };

        assert_eq!(query.r#match(&tree), true);
    }

    #[test]
    fn test_match_noimage() {
        let query = SearchQuery::from_string("thuja noimage");

        let tree1 = TreeRecord {
            species: "Thuja plicata".to_string(),
            thumbnail_id: None,
            ..default_tree()
        };

        assert_eq!(query.r#match(&tree1), true);

        let tree2 = TreeRecord {
            species: "Thuja plicata".to_string(),
            thumbnail_id: Some(1),
            ..default_tree()
        };

        assert_eq!(
            query.r#match(&tree2),
            false,
            "tree has thumbnail and must not match"
        );
    }

    #[test]
    fn test_match_nometrics() {
        let query = SearchQuery::from_string("nometrics");

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                height: None,
                circumference: Some(2.0),
                diameter: Some(3.0),
                ..default_tree()
            })
        );

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                height: Some(1.0),
                circumference: None,
                diameter: Some(3.0),
                ..default_tree()
            })
        );

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                height: Some(1.0),
                circumference: Some(2.0),
                diameter: None,
                ..default_tree()
            })
        );

        assert_eq!(
            false,
            query.r#match(&TreeRecord {
                height: Some(1.0),
                circumference: Some(2.0),
                diameter: Some(3.0),
                ..default_tree()
            }),
            "the tree has all metrics and must not match"
        );
    }

    #[test]
    fn test_sick() {
        let query = SearchQuery::from_string("sick");

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                state: "sick".to_string(),
                ..default_tree()
            })
        );

        assert_eq!(
            false,
            query.r#match(&TreeRecord {
                state: "healthy".to_string(),
                ..default_tree()
            })
        );
    }

    #[test]
    fn test_dead() {
        let query = SearchQuery::from_string("dead");

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                state: "dead".to_string(),
                ..default_tree()
            })
        );

        assert_eq!(
            false,
            query.r#match(&TreeRecord {
                state: "healthy".to_string(),
                ..default_tree()
            })
        );
    }

    #[test]
    fn test_deformed() {
        let query = SearchQuery::from_string("deformed");

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                state: "deformed".to_string(),
                ..default_tree()
            })
        );

        assert_eq!(
            false,
            query.r#match(&TreeRecord {
                state: "healthy".to_string(),
                ..default_tree()
            })
        );
    }

    #[test]
    fn test_healthy() {
        let query = SearchQuery::from_string("healthy");

        assert_eq!(
            true,
            query.r#match(&TreeRecord {
                state: "healthy".to_string(),
                ..default_tree()
            })
        );

        assert_eq!(
            false,
            query.r#match(&TreeRecord {
                state: "sick".to_string(),
                ..default_tree()
            })
        );
    }
}
