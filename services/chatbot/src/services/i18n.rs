use fluent::{FluentArgs, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use std::collections::HashMap;
use std::fs;
use unic_langid::LanguageIdentifier;

pub struct I18n {
    bundles: HashMap<String, fluent::bundle::FluentBundle<FluentResource, IntlLangMemoizer>>,
    default_lang: String,
}

impl I18n {
    pub fn new() -> Self {
        let mut bundles = HashMap::new();
        let locales = ["en", "ru"];

        for lang in locales {
            let path = format!("locales/{}.ftl", lang);
            let source =
                fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read {}", path));
            let resource = FluentResource::try_new(source).expect("Failed to parse FTL");

            let lang_id: LanguageIdentifier = lang.parse().expect("Failed to parse language ID");
            let mut bundle = fluent::bundle::FluentBundle::new_concurrent(vec![lang_id]);
            bundle
                .add_resource(resource)
                .expect("Failed to add resource");

            bundles.insert(lang.to_string(), bundle);
        }

        Self {
            bundles,
            default_lang: "en".to_string(),
        }
    }

    pub fn tr(&self, key: &str, lang: &str, args: Option<&FluentArgs>) -> String {
        let lang = if self.bundles.contains_key(lang) {
            lang
        } else {
            &self.default_lang
        };

        let bundle = self.bundles.get(lang).unwrap();
        let msg = bundle
            .get_message(key)
            .unwrap_or_else(|| panic!("Message {} not found", key));
        let pattern = msg
            .value()
            .unwrap_or_else(|| panic!("Message {} has no value", key));

        let mut errors = vec![];
        let result = bundle.format_pattern(pattern, args, &mut errors);

        result.to_string()
    }
}
