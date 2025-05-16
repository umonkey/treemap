use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use html_escape::encode_double_quoted_attribute_to_string;
use std::sync::Arc;
use log::{debug, error};
use tokio::fs;

pub struct TreePageHandler {
    trees: Arc<TreeRepository>,
}

impl TreePageHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<String> {
        debug!("Injecting meta for tree {tree_id}.");

        let mut html = self.get_tree_meta(tree_id).await?;
        html = self.inject_meta(&html).await?;

        Ok(html)
    }

    async fn get_tree_meta(&self, tree_id: u64) -> Result<String> {
        match self.trees.get(tree_id).await? {
            Some(tree) => self.get_existing_meta(tree).await,
            None => self.get_notfound_meta().await,
        }
    }

    async fn get_existing_meta(&self, tree: TreeRecord) -> Result<String> {
        let mut html = String::new();

        let title = Self::format_title(&tree);
        let description = Self::format_description(&tree);
        let url = format!("https://yerevan.treemaps.app/tree/{}", tree.id);

        html.push_str(format!("<title>{}</title>", Self::escape(&title)).as_str());
        html.push_str(format!("<meta name=\"description\" content=\"{}\">", Self::escape(&description)).as_str());

        html.push_str(format!("<meta name=\"og:url\" content=\"{}\">", Self::escape(&url)).as_str());
        html.push_str(format!("<meta name=\"og:title\" content=\"{}\">", Self::escape(&title)).as_str());
        html.push_str(format!("<meta name=\"og:description\" content=\"{}\">", Self::escape(&description)).as_str());

        if let Some(image) = tree.thumbnail_id {
            html.push_str(format!("<meta name=\"og:image\" content=\"https://yerevan.treemaps.app/v1/files/{}.jpg\">", image).as_str());
        }

        html.push_str("<meta name=\"twitter:card\" content=\"summary_large_image\">");
        html.push_str("<meta property=\"twitter:domain\" content=\"yerevan.treemaps.app\">");
        html.push_str(format!("<meta property=\"twitter:url\" content=\"{}\">", Self::escape(&url)).as_str());
        html.push_str(format!("<meta name=\"twitter:title\" content=\"{}\">", Self::escape(&title)).as_str());
        html.push_str(format!("<meta name=\"twitter:description\" content=\"{}\">", Self::escape(&description)).as_str());

        if let Some(image) = tree.thumbnail_id {
            html.push_str(format!("<meta name=\"twitter:image\" content=\"https://yerevan.treemaps.app/v1/files/{}.jpg\">", image).as_str());
        }

        Ok(html)
    }

    async fn get_notfound_meta(&self) -> Result<String> {
        let html = "<title>Tree not found</title>";
        Ok(html.to_string())
    }

    fn format_title(tree: &TreeRecord) -> String {
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

    fn format_description(tree: &TreeRecord) -> String {
        match tree.state.as_str() {
            "gone" => format!("There once was a {} tree at {}, {}.", tree.species, tree.lat, tree.lon),
            "stomp" => format!("What's left of a {} tree at {}, {}.", tree.species, tree.lat, tree.lon),
            state => format!("A {} tree at {}, {}.", state, tree.lat, tree.lon),
        }.to_string()
    }

    fn escape(text: &str) -> String {
        let mut value = String::new();
        encode_double_quoted_attribute_to_string(text, &mut value);
        value
    }

    async fn inject_meta(&self, html: &str) -> Result<String>
    {
        let path = "static/index.html";

        let body = fs::read_to_string(path).await.map_err(|e| {
            error!("Error reading file: {:?}", e);
            Error::FileNotFound
        })?;

        Ok(body.replace("<!-- meta -->", html))
    }
}

impl Locatable for TreePageHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
