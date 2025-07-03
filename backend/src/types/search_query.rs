//! This structure represents a search query.
//!
//! We parse it to extract the words to search for, and flags to disable certain
//! search features.

use crate::types::TreeRecord;
use crate::utils::{get_timestamp, split_words};

#[derive(Debug, Default)]
pub struct SearchQuery {
    pub words: Vec<String>,
    pub incomplete: bool,
    pub nometrics: bool,
    pub nocirc: bool,
    pub noimages: bool,
    pub hasaddr: bool,
    pub noaddr: bool,
    pub noheight: bool,
    pub nodiameter: bool,
    pub hasimages: bool,
    pub sick: bool,
    pub dead: bool,
    pub deformed: bool,
    pub healthy: bool,
    pub stump: bool,
    pub gone: bool,
    pub unknown: bool,
    pub all: bool,
    pub address: Option<String>,
    pub species: Option<String>,

    pub added_by_me: bool,

    pub added_after: Option<u64>,
    pub updated_after: Option<u64>,
}

impl SearchQuery {
    pub fn from_string(query: &str) -> SearchQuery {
        let mut res = SearchQuery {
            ..Default::default()
        };

        for word in split_words(query.to_lowercase().as_str()) {
            if word.contains("nometric") {
                res.nometrics = true;
            } else if word.contains("no:circumference") {
                res.nocirc = true;
            } else if word.contains("no:diameter") {
                res.nodiameter = true;
            } else if word.contains("noimage") || word.contains("nophoto") {
                res.noimages = true;
            } else if word.contains("hasimage") || word.contains("hasphoto") {
                res.hasimages = true;
            } else if word.contains("healthy") {
                res.healthy = true;
            } else if word.contains("deformed") {
                res.deformed = true;
            } else if word.contains("sick") {
                res.sick = true;
            } else if word.contains("dead") {
                res.dead = true;
            } else if word.contains("stump") {
                res.stump = true;
            } else if word.contains("gone") {
                res.gone = true;
            } else if word.contains("state:unknown") {
                res.unknown = true;
            } else if word.contains("incomplete") {
                res.incomplete = true;
            } else if word == "has:addr" {
                res.hasaddr = true;
            } else if word == "no:addr" {
                res.noaddr = true;
            } else if word == "no:height" {
                res.noheight = true;
            } else if word == "added:me" {
                res.added_by_me = true;
            } else if let Some(value) = word.strip_prefix("added:") {
                if let Ok(value) = value.parse::<u64>() {
                    res.added_after = Some(get_timestamp() - value);
                }
            } else if let Some(value) = word.strip_prefix("updated:") {
                if let Ok(value) = value.parse::<u64>() {
                    res.updated_after = Some(get_timestamp() - value);
                }
            } else if word.contains("all") {
                res.all = true;
            } else if let Some(value) = word.strip_prefix("addr:") {
                res.address = Some(value.to_string());
            } else if let Some(value) = word.strip_prefix("species:") {
                res.species = Some(value.to_string());
            } else {
                res.words.push(word.to_string().to_lowercase());
            }
        }

        res
    }

    pub fn r#match(&self, tree: &TreeRecord, user_id: u64) -> bool {
        if !self.match_text(tree) {
            return false;
        }

        if self.noimages && tree.thumbnail_id.is_some() {
            return false;
        }

        if self.hasimages && tree.thumbnail_id.is_none() {
            return false;
        }

        if self.nometrics
            && tree.height.is_some()
            && tree.circumference.is_some()
            && tree.diameter.is_some()
        {
            return false;
        }

        if self.nocirc && tree.circumference.unwrap_or(0.0) > 0.0 {
            return false;
        }

        if self.nodiameter && tree.diameter.unwrap_or(0.0) > 0.0 {
            return false;
        }

        if self.noheight && tree.height.unwrap_or(0.0) > 0.0 {
            return false;
        }

        if self.hasaddr && tree.address.is_none() {
            return false;
        }

        if self.noaddr && tree.address.is_some() {
            return false;
        }

        if let Some(a1) = &self.address {
            if let Some(a2) = &tree.address {
                if a1.to_lowercase() != a2.to_lowercase() {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(value) = &self.species {
            if !tree.species.to_lowercase().contains(value) {
                return false;
            }
        }

        if self.added_by_me && tree.added_by != user_id {
            return false;
        }

        if let Some(value) = &self.added_after {
            if tree.added_at < *value {
                return false;
            }
        }

        if let Some(value) = &self.updated_after {
            if tree.updated_at < *value {
                return false;
            }
        }

        if !self.all {
            if !self.sick
                && !self.dead
                && !self.deformed
                && !self.healthy
                && !self.stump
                && !self.gone
                && !self.unknown
                && tree.state == "gone"
            {
                return false;
            }

            if self.healthy && tree.state != "healthy" {
                return false;
            }

            if self.deformed && tree.state != "deformed" {
                return false;
            }

            if self.sick && tree.state != "sick" {
                return false;
            }

            if self.dead && tree.state != "dead" {
                return false;
            }

            if self.stump && tree.state != "stump" {
                return false;
            }

            if self.gone && tree.state != "gone" {
                return false;
            }

            if self.unknown && tree.state != "unknown" {
                return false;
            }
        }

        if self.incomplete && Self::is_tree_incomplete(tree) {
            return true;
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

    fn is_tree_incomplete(tree: &TreeRecord) -> bool {
        tree.state == "unknown"
            || tree.height.is_none()
            || tree.circumference.is_none()
            || tree.diameter.is_none()
            || tree.thumbnail_id.is_none()
    }

    fn get_tree_text(tree: &TreeRecord) -> String {
        let mut words: Vec<String> = Vec::new();
        words.push(tree.species.to_lowercase());

        if let Some(value) = &tree.notes {
            words.push(value.to_lowercase());
        }

        if let Some(value) = &tree.address {
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
            state: "unknown".to_string(),
            added_at: 0,
            updated_at: 0,
            added_by: 0,
            thumbnail_id: None,
            year: None,
            address: None,
            ..Default::default()
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
    }

    #[test]
    fn test_match_text() {
        let query = SearchQuery::from_string("thuja");

        let tree = TreeRecord {
            species: "Thuja plicata".to_string(),
            ..default_tree()
        };

        assert_eq!(query.r#match(&tree, 0), true);
    }

    #[test]
    fn test_match_noimage() {
        let query = SearchQuery::from_string("thuja noimage");

        let tree1 = TreeRecord {
            species: "Thuja plicata".to_string(),
            thumbnail_id: None,
            ..default_tree()
        };

        assert_eq!(query.r#match(&tree1, 0), true);

        let tree2 = TreeRecord {
            species: "Thuja plicata".to_string(),
            thumbnail_id: Some(1),
            ..default_tree()
        };

        assert_eq!(
            query.r#match(&tree2, 0),
            false,
            "tree has thumbnail and must not match"
        );
    }

    #[test]
    fn test_match_nometrics() {
        let query = SearchQuery::from_string("nometrics");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    height: None,
                    circumference: Some(2.0),
                    diameter: Some(3.0),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    height: Some(1.0),
                    circumference: None,
                    diameter: Some(3.0),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    height: Some(1.0),
                    circumference: Some(2.0),
                    diameter: None,
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    height: Some(1.0),
                    circumference: Some(2.0),
                    diameter: Some(3.0),
                    ..default_tree()
                },
                0
            ),
            "the tree has all metrics and must not match"
        );
    }

    #[test]
    fn test_healthy() {
        let query = SearchQuery::from_string("healthy");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "sick".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_deformed() {
        let query = SearchQuery::from_string("deformed");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "deformed".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_sick() {
        let query = SearchQuery::from_string("Sick");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "sick".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_dead() {
        let query = SearchQuery::from_string("dead");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "dead".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_stump() {
        let query = SearchQuery::from_string("stump");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "stump".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_gone() {
        let query = SearchQuery::from_string("gone");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "gone".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_non_gone() {
        let query = SearchQuery::from_string("");

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "gone".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_unknown() {
        let query = SearchQuery::from_string("state:unknown");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "unknown".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_all() {
        let query = SearchQuery::from_string("all");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "gone".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    state: "healthy".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_notes() {
        let query = SearchQuery::from_string("osm");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    notes: Some("Imported from OSM".to_string()),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    notes: Some("Big tree".to_string()),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_nocirc() {
        let query = SearchQuery::from_string("no:circumference");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    circumference: None,
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    circumference: Some(2.0),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_hasaddr() {
        let query = SearchQuery::from_string("has:addr");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: Some("test".to_string()),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: None,
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_noaddr() {
        let query = SearchQuery::from_string("no:addr");

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: Some("test".to_string()),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: None,
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_address() {
        let query = SearchQuery::from_string("addr:\"Some Street\"");

        assert_eq!(Some("some street"), query.address.as_deref());

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: Some("Some Street".to_string()),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: Some("other street".to_string()),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    address: None,
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_species() {
        let query = SearchQuery::from_string("species:\"Thuja plicata\"");

        assert_eq!(Some("thuja plicata"), query.species.as_deref());

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    species: "Thuja plicata".to_string(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    species: "Thuja orientalis".to_string(),
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_added_by_me() {
        let query = SearchQuery::from_string("added:me");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    added_by: 1,
                    ..default_tree()
                },
                1
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    added_by: 1,
                    ..default_tree()
                },
                2
            )
        );
    }

    #[test]
    fn test_added_after() {
        let query = SearchQuery::from_string("added:3600");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    added_at: get_timestamp(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    added_at: get_timestamp() - 3601,
                    ..default_tree()
                },
                0
            )
        );
    }

    #[test]
    fn test_updated_after() {
        let query = SearchQuery::from_string("updated:3600");

        assert_eq!(
            true,
            query.r#match(
                &TreeRecord {
                    updated_at: get_timestamp(),
                    ..default_tree()
                },
                0
            )
        );

        assert_eq!(
            false,
            query.r#match(
                &TreeRecord {
                    updated_at: get_timestamp() - 3601,
                    ..default_tree()
                },
                0
            )
        );
    }
}
