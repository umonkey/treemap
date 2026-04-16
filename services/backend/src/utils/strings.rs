use xml::escape::escape_str_attribute;

pub fn split_words(text: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut word = String::new();

    let mut in_quote = false;

    for c in text.chars() {
        if c == '"' {
            in_quote = !in_quote;
            continue;
        }

        if c == ' ' && !in_quote {
            if !word.is_empty() {
                res.push(word.clone());
                word.clear();
            }
        } else {
            word.push(c);
        }
    }

    if !word.is_empty() {
        res.push(word);
    }

    res
}

pub fn osm_tag(key: &str, value: &str) -> String {
    format!(
        r#"<tag k="{}" v="{}" />"#,
        escape_str_attribute(key),
        escape_str_attribute(value)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let text = "Hello, world!";
        let words = split_words(text);
        assert_eq!(words, vec!["Hello,", "world!"]);
    }

    #[test]
    fn test_quoted() {
        let text = "noimage addr:\"Nairi Zaryan Street\"";
        let words = split_words(text);
        assert_eq!(words, vec!["noimage", "addr:Nairi Zaryan Street"]);
    }

    #[test]
    fn test_unfinished() {
        let text = "noimage addr:\"Nairi Zaryan Street";
        let words = split_words(text);
        assert_eq!(words, vec!["noimage", "addr:Nairi Zaryan Street"]);
    }

    #[test]
    fn test_multiple() {
        let text = "noimage \"foo bar\" \"bang bang\"";
        let words = split_words(text);
        assert_eq!(words, vec!["noimage", "foo bar", "bang bang"]);
    }
}
