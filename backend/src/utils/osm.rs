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
