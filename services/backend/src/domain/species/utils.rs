//! This module contains simple functions.
//! The idea is to make them testable.

fn capitalized(value: &str) -> String {
    let mut chars = value.chars();

    let capitalized = match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    };

    capitalized
}

pub fn get_species_name(stored: &str) -> Option<String> {
    let name = stored.to_lowercase();
    let parts: Vec<&str> = name.split_whitespace().collect();

    // Genus
    if parts.len() < 2 {
        return None;
    }

    let first_two: Vec<&str> = parts.into_iter().take(2).collect();
    let species = first_two.join(" ");

    if species.contains("unknown") {
        return None;
    }

    Some(capitalized(&species))
}

pub fn get_genus_name(stored: &str) -> Option<String> {
    let name = stored.to_lowercase();
    let parts: Vec<&str> = name.split_whitespace().collect();

    let first: Vec<&str> = parts.into_iter().take(1).collect();
    let genus = first.join(" ");

    if genus.contains("unknown") {
        return None;
    }

    Some(capitalized(&genus))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalized() {
        let value = capitalized("acer campestre");
        assert_eq!(value, "Acer campestre");
    }

    #[test]
    fn test_get_species_name() {
        let value = get_species_name("unknown tree");
        assert!(value.is_none());

        let value = get_species_name("acer");
        assert!(value.is_none());

        let value = get_species_name("Acer Campestre");
        assert_eq!(value, Some("Acer campestre".to_string()));

        let value = get_species_name("Acer Campestre x truncatum");
        assert_eq!(value, Some("Acer campestre".to_string()));
    }

    #[test]
    fn test_get_genus_name() {
        let value = get_genus_name("unknown tree");
        assert!(value.is_none());

        let value = get_genus_name("acer");
        assert_eq!(value, Some("Acer".to_string()));

        let value = get_genus_name("Acer Campestre");
        assert_eq!(value, Some("Acer".to_string()));

        let value = get_genus_name("Acer Campestre x truncatum");
        assert_eq!(value, Some("Acer".to_string()));
    }
}
