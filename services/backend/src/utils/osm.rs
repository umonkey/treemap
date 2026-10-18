// OSM rounds to 7 decimal points which is 1 cm accuracy.
// @docs https://support.garmin.com/en-US/?faq=hRMBoCTy5a7HqVkxukhHd8
pub fn osm_round_coord(coord: f64) -> f64 {
    (coord * 10_000_000.0).round() / 10_000_000.0
}

pub fn get_osm_genus(species: &str) -> Option<String> {
    if species.to_lowercase().contains("unknown") {
        return None;
    }

    let mut parts = species.split_whitespace();
    parts.next().map(|value| value.to_string())
}

pub fn get_osm_species(species: &str) -> Option<String> {
    if species.to_lowercase().contains("unknown") {
        return None;
    }

    if !species.contains(" ") {
        return None;
    }

    Some(species.to_string())
}

// Build an Overpass query for all tree nodes in the given bbox.
// The bbox is in the [west, south, east, north] order, while Overpass
// expects (south, west, north, east).
pub fn build_overpass_query(bbox: [f64; 4]) -> String {
    let [west, south, east, north] = bbox;

    format!("[out:json];node[natural=tree]({south},{west},{north},{east});out meta;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_overpass_query() {
        let query = build_overpass_query([44.0, 40.0, 44.5, 40.5]);

        assert_eq!(
            query,
            "[out:json];node[natural=tree](40,44,40.5,44.5);out meta;"
        );
    }
}
