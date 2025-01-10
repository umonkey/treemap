use crate::services::*;
use crate::types::*;
use crate::utils::*;
use crate::utils::{get_app_name, get_app_version};
use log::{debug, error, info};
use reqwest::Client;
use serde_json::Value;
use xml::escape::escape_str_attribute;

pub struct OsmClient {
    client: Client,
}

impl OsmClient {
    pub async fn create_changeset(&self, comment: &str) -> Result<u64> {
        let changeset = OsmChangeset {
            created_by: format!("{}/{}", get_app_name(), get_app_version()),
            host: "https://yerevan.treemaps.app/".to_string(),
            bot: true,
            source: "survey".to_string(),
            comment: comment.to_string(),
        };

        let url = "https://api.openstreetmap.org/api/0.6/changeset/create";
        let body: String = format!("<osm>{}</osm>", changeset);

        let res = self.put(url, body.as_str()).await?;

        match res.parse::<u64>() {
            Ok(id) => {
                info!("Created OSM changeset {}", id);
                Ok(id)
            }

            Err(e) => {
                error!("Error parsing OSM changeset ID: {:?}", e);
                Err(Error::OsmExchange)
            }
        }
    }

    pub async fn close_changeset(&self, id: u64) -> Result<()> {
        let url = format!(
            "https://api.openstreetmap.org/api/0.6/changeset/{}/close",
            id
        );
        self.put(url.as_str(), "").await?;
        Ok(())
    }

    pub async fn create_tree(&self, changeset_id: u64, tree: &TreeRecord) -> Result<u64> {
        let url = "https://api.openstreetmap.org/api/0.6/node/create";

        let xml = format!(
            "<osm>{}</osm>",
            self.format_new_tree_node(tree, changeset_id)?
        );
        let res = self.put(url, &xml).await?;

        match res.parse::<u64>() {
            Ok(id) => {
                info!("Created OSM node {} for tree {}", id, tree.id);
                Ok(id)
            }

            Err(e) => {
                error!("Error parsing OSM node id: {:?}", e);
                Err(Error::OsmExchange)
            }
        }
    }

    pub async fn update_tree(&self, changeset_id: u64, element: &OsmElement) -> Result<()> {
        let url = format!("https://api.openstreetmap.org/api/0.6/node/{}", element.id);

        let xml = format!(
            "<osm>{}</osm>",
            self.format_node_update(changeset_id, element)?
        );

        self.put(&url, &xml).await?;

        info!("OSM node {} updated.", element.id);

        Ok(())
    }

    pub async fn get_token(&self, code: &str) -> Result<String> {
        let json = self
            .request_json(&format!(
                "https://api.openstreetmap.org/oauth/token?grant_type=authorization_code&code={}&client_id={}&client_secret={}&redirect_uri={}",
                code,
                get_osm_client_id()?,
                get_osm_client_secret()?,
                get_osm_redirect_uri()?
            ))
            .await?;

        match json["access_token"].as_str() {
            Some(token) => Ok(token.to_string()),

            None => {
                error!("OSM response does not contain access_token.");
                Err(Error::OsmExchange)
            }
        }
    }

    #[allow(unused)]
    pub async fn get_nodes(&self, ids: &[u64]) -> Result<Vec<OsmElement>> {
        let json = self
            .request_json(&format!(
                "https://api.openstreetmap.org/api/0.6/nodes.json?nodes={}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ))
            .await?;

        let raw_elements = match json["elements"].as_array() {
            Some(elements) => elements,

            None => {
                error!("OSM response does not contain elements array.");
                return Err(Error::OsmExchange);
            }
        };

        let mut res: Vec<OsmElement> = Vec::new();

        for raw_element in raw_elements {
            match serde_json::from_value::<OsmElement>(raw_element.clone()) {
                Ok(element) => res.push(element),

                Err(e) => {
                    error!("Error parsing OSM element: {:?}", e);
                }
            };
        }

        Ok(res)
    }

    pub async fn get_node(&self, id: u64) -> Result<Option<OsmElement>> {
        let url = format!("https://api.openstreetmap.org/api/0.6/node/{}.json", id);

        let response = match self.client.get(url.to_string()).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error querying OSM API: {}; URL: {}", e, url);
                return Err(Error::OsmExchange);
            }
        };

        if response.status() == 410 {
            debug!("OSM node {} is gone.", id);
            return Ok(None);
        }

        if response.status() == 404 {
            debug!("OSM node {} not found.", id);
            return Ok(None);
        }

        if response.status() != 200 {
            error!(
                "OSM API GET failed with status: {}; URL: {}",
                response.status(),
                url
            );
            return Err(Error::OsmExchange);
        }

        let json: Value = match response.json().await {
            Ok(json) => json,

            Err(e) => {
                error!("Error parsing OSM API response JSON: {:?}", e);
                return Err(Error::OsmExchange);
            }
        };

        let raw_elements = match json["elements"].as_array() {
            Some(elements) => elements,

            None => {
                error!("OSM response does not contain elements array.");
                return Err(Error::OsmExchange);
            }
        };

        for raw_element in raw_elements {
            match serde_json::from_value::<OsmElement>(raw_element.clone()) {
                Ok(element) => return Ok(Some(element)),

                Err(e) => {
                    error!("Error parsing OSM element: {:?}", e);
                }
            };
        }

        Ok(None)
    }

    async fn put(&self, url: &str, body: &str) -> Result<String> {
        let token = get_osm_token()?;

        debug!("OSM PUT: {}; body: {}", url, body);

        let response = match self
            .client
            .put(url)
            .header("Authorization", format!("bearer {}", token))
            .body(body.to_string())
            .send()
            .await
        {
            Ok(response) => response,

            Err(e) => {
                error!("Error querying OSM API: {}", e);
                return Err(Error::OsmExchange);
            }
        };

        if response.status() != 200 {
            error!(
                "OSM API PUT failed with status: {}; URL: {}; body: {}",
                response.status(),
                url,
                body
            );
            return Err(Error::OsmExchange);
        }

        response.text().await.map_err(|e| {
            error!("Error parsing OSM API response text: {:?}", e);
            Error::OsmExchange
        })
    }

    async fn request_json(&self, url: &str) -> Result<Value> {
        let response = match self.client.get(url).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error querying OSM API: {}", e);
                return Err(Error::OsmExchange);
            }
        };

        if response.status() != 200 {
            error!(
                "OSM API GET failed with status: {}; URL: {}",
                response.status(),
                url
            );
            return Err(Error::OsmExchange);
        }

        let json: Value = match response.json().await {
            Ok(json) => json,

            Err(e) => {
                error!("Error parsing OSM API response JSON: {:?}", e);
                return Err(Error::OsmExchange);
            }
        };

        Ok(json)
    }

    fn format_new_tree_node(&self, tree: &TreeRecord, changeset_id: u64) -> Result<String> {
        let mut xml = format!(
            "<node changeset=\"{}\" lat=\"{}\" lon=\"{}\">",
            changeset_id, tree.lat, tree.lon
        );
        xml.push_str("<tag k=\"natural\" v=\"tree\" />");

        if let Some(value) = tree.get_genus() {
            xml.push_str(&format!(
                "<tag k=\"genus\" v=\"{}\" />",
                escape_str_attribute(value.as_str())
            ));
        }

        if let Some(value) = tree.get_full_species() {
            xml.push_str(&format!(
                "<tag k=\"species\" v=\"{}\" />",
                escape_str_attribute(value.as_str())
            ));
        }

        if let Some(value) = tree.height {
            if value > 0.0 {
                xml.push_str(&format!("<tag k=\"height\" v=\"{}\" />", value));
            }
        }

        if let Some(value) = tree.circumference {
            if value > 0.0 {
                xml.push_str(&format!("<tag k=\"circumference\" v=\"{}\" />", value));
            }
        }

        if let Some(value) = tree.diameter {
            if value > 0.0 {
                xml.push_str(&format!("<tag k=\"diameter_crown\" v=\"{}\" />", value));
            }
        }

        // https://wiki.openstreetmap.org/wiki/Key:image
        let url = format!("https://yerevan.treemaps.app/tree/{}", tree.id);
        xml.push_str(&format!("<tag k=\"image\" v=\"{}\" />", url));

        xml.push_str("</node>");
        Ok(xml)
    }

    fn format_node_update(&self, changeset_id: u64, element: &OsmElement) -> Result<String> {
        let mut xml = format!(
            "<node changeset=\"{}\" id=\"{}\" version=\"{}\" lat=\"{}\" lon=\"{}\">",
            changeset_id, element.id, element.version, element.lat, element.lon
        );

        for (k, v) in &element.tags {
            xml.push_str(osm_tag(k, v).as_str());
        }

        xml.push_str("</node>");
        Ok(xml)
    }
}

impl Locatable for OsmClient {
    fn create(_locator: &Locator) -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::new(),
        })
    }
}
