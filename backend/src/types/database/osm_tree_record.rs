const DEFAULT_SPECIES: &str = "Unknown tree";

#[derive(Clone, Debug, PartialEq)]
pub struct OsmTreeRecord {
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub genus: Option<String>,
    pub species: Option<String>,
    pub species_wikidata: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter_crown: Option<f64>,
}

impl OsmTreeRecord {
    pub fn get_species(&self) -> String {
        if let Some(value) = &self.species {
            return value.to_string();
        }

        if let Some(value) = &self.genus {
            return value.to_string();
        }

        DEFAULT_SPECIES.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default() -> OsmTreeRecord {
        OsmTreeRecord {
            id: 1,
            lat: 0.0,
            lon: 0.0,
            genus: None,
            species: None,
            species_wikidata: None,
            height: None,
            circumference: None,
            diameter_crown: None,
        }
    }

    #[test]
    fn test_get_species() {
        let tree = OsmTreeRecord {
            species: Some("Quercus robur".to_string()),
            ..default()
        };

        assert_eq!(tree.get_species(), "Quercus robur");
    }

    #[test]
    fn test_get_species_genus() {
        let tree = OsmTreeRecord {
            genus: Some("Quercus".to_string()),
            ..default()
        };

        assert_eq!(tree.get_species(), "Quercus");
    }

    #[test]
    fn test_get_species_empty() {
        assert_eq!(default().get_species(), DEFAULT_SPECIES);
    }
}
