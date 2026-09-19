use crate::domain::tree::{Tree, TreeState};
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use html_escape::encode_double_quoted_attribute_to_string;
use log::{debug, error};
use regex::Regex;
use std::sync::{LazyLock, OnceLock};
use tokio::fs;

static INDEX_TEMPLATE: OnceLock<String> = OnceLock::new();

/// Paths that should not be indexed by search engines.
/// Matched against the request path (without the query string).
const NOINDEX_PATTERNS: &[&str] = &[
    r"^/tree(/.*)?$",
    r"^/alert(/.*)?$",
    r"^/water(/.*)?$",
    r"^/panoramas(/.*)?$",
    r"^/learn/?$",
    r"^/updates/?$",
    r"^/search/?$",
    r"^/layers/?$",
    r"^/profile(/.*)?$",
    r"^/saved/?$",
    r"^/add(/.*)?$",
];

static NOINDEX_REGEXES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    NOINDEX_PATTERNS
        .iter()
        .map(|pattern| Regex::new(pattern).expect("invalid noindex regex"))
        .collect()
});

pub struct MetaService {}

impl MetaService {
    pub async fn get_tree(&self, tree: &Tree) -> Result<String> {
        let mut html = String::new();

        let title = Self::format_title(tree);
        let description = Self::format_description(tree);
        let url = format!("https://yerevan.treemaps.app/tree/{}", tree.id);

        html.push_str(format!("<title>{}</title>", Self::escape(&title)).as_str());
        html.push_str(
            format!(
                "<meta name=\"description\" content=\"{}\">",
                Self::escape(&description)
            )
            .as_str(),
        );

        html.push_str(
            format!("<meta name=\"og:url\" content=\"{}\">", Self::escape(&url)).as_str(),
        );
        html.push_str(
            format!(
                "<meta name=\"og:title\" content=\"{}\">",
                Self::escape(&title)
            )
            .as_str(),
        );
        html.push_str(
            format!(
                "<meta name=\"og:description\" content=\"{}\">",
                Self::escape(&description)
            )
            .as_str(),
        );

        if let Some(image) = tree.thumbnail_id {
            html.push_str(format!("<meta name=\"og:image\" content=\"https://yerevan.treemaps.app/v1/files/{image}.jpg\">").as_str());
        }

        html.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\">");
        html.push_str("<meta property=\"twitter:domain\" content=\"yerevan.treemaps.app\">");
        html.push_str(
            format!(
                "<meta property=\"twitter:url\" content=\"{}\">",
                Self::escape(&url)
            )
            .as_str(),
        );
        html.push_str(
            format!(
                "<meta name=\"twitter:title\" content=\"{}\">",
                Self::escape(&title)
            )
            .as_str(),
        );
        html.push_str(
            format!(
                "<meta name=\"twitter:description\" content=\"{}\">",
                Self::escape(&description)
            )
            .as_str(),
        );

        if let Some(image) = tree.thumbnail_id {
            html.push_str(format!("<meta name=\"twitter:image\" content=\"https://yerevan.treemaps.app/v1/files/{image}.jpg\">").as_str());
        }

        self.inject_meta(&html).await
    }

    #[allow(unused)]
    pub async fn get_notfound_meta(&self) -> Result<String> {
        let html = "<title>Tree not found</title>";
        Ok(html.to_string())
    }

    fn format_title(tree: &Tree) -> String {
        if let Some(addr) = &tree.address {
            return format!("{} on {}", Self::escape(&tree.species), Self::escape(addr));
        }

        format!(
            "{} at {}, {}",
            Self::escape(&tree.species),
            tree.lat,
            tree.lon
        )
    }

    fn format_description(tree: &Tree) -> String {
        match tree.state {
            TreeState::Gone => format!(
                "There once was a {} tree at {}, {}.",
                tree.species, tree.lat, tree.lon
            ),
            TreeState::Stump => format!(
                "What's left of a {} tree at {}, {}.",
                tree.species, tree.lat, tree.lon
            ),
            state => format!("A {} tree at {}, {}.", state.as_str(), tree.lat, tree.lon),
        }
        .to_string()
    }

    fn escape(text: &str) -> String {
        let mut value = String::new();
        encode_double_quoted_attribute_to_string(text, &mut value);
        value
    }

    /// Injects the robots meta tag appropriate for `path` into `html`.
    pub fn inject_robots(&self, html: &str, path: &str) -> String {
        if !Self::is_noindex(path) {
            debug!("Not injecting meta in {path}");
            return html.to_string();
        }

        html.replace(
            "</head>",
            "<meta name=\"robots\" content=\"noindex\"></head>",
        )
    }

    fn is_noindex(path: &str) -> bool {
        NOINDEX_REGEXES.iter().any(|regex| regex.is_match(path))
    }

    async fn inject_meta(&self, html: &str) -> Result<String> {
        let path = "static/index.html";

        let body = if let Some(template) = INDEX_TEMPLATE.get() {
            template.clone()
        } else {
            let content = fs::read_to_string(path).await.map_err(|e| {
                error!("Error reading file: {e:?}");
                Error::FileNotFound
            })?;

            let _ = INDEX_TEMPLATE.set(content.clone());
            content
        };

        Ok(body.replace("<!-- meta -->", html))
    }
}

impl Injectable for MetaService {
    fn inject(_ctx: &dyn Context) -> Result<Self> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inject_robots_noindex() {
        let service = MetaService {};
        let html = "<html><head></head><body></body></html>";

        let result = service.inject_robots(html, "/tree/123");

        assert!(result.contains("<meta name=\"robots\" content=\"noindex\">"));
        assert!(result.contains("</head>"));
    }

    #[test]
    fn test_inject_robots_indexable() {
        let service = MetaService {};
        let html = "<html><head></head><body></body></html>";

        let result = service.inject_robots(html, "/stats");

        assert_eq!(result, html);
    }

    #[test]
    fn test_inject_robots_excluded_paths() {
        let service = MetaService {};

        for path in [
            "/tree/123",
            "/tree/123/preview",
            "/tree/123/edit",
            "/alert/123",
            "/alert/123/preview",
            "/water/123",
            "/water/123/move",
            "/panoramas/123",
            "/learn",
            "/updates",
            "/search",
            "/layers",
        ] {
            let result = service.inject_robots("<head></head>", path);

            assert!(result.contains("noindex"), "expected noindex for {path}");
        }
    }

    #[test]
    fn test_inject_robots_indexable_paths() {
        let service = MetaService {};

        for path in ["/", "/stats", "/privacy"] {
            let result = service.inject_robots("<head></head>", path);

            assert!(!result.contains("noindex"), "expected indexable for {path}");
        }
    }
}
