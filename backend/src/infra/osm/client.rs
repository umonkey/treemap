use crate::config::{Config, Secrets};
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
    client_id: Option<String>,
    redirect_uri: Option<String>,
    hashtag: Option<String>,
    activity: Option<String>,

    // This is the main API access token.
    // It is optional so that we don't fail to start the API when OSM sync is not configured.
    // FIXME: the API should not load this in the first place.
    osm_token: Option<String>,

    // This is the app password, used to retrieve a token.
    osm_client_secret: Option<String>,
}

impl OsmClient {
    pub async fn create_changeset(&self, comment: &str) -> Result<u64> {
        let changeset = OsmChangeset {
            created_by: format!("{}/{}", get_app_name(), get_app_version()),
            host: "https://yerevan.treemaps.app/".to_string(),
            bot: true,
            source: "survey".to_string(),
            comment: self.format_changeset_comment(comment),
        };

        let url = "https://api.openstreetmap.org/api/0.6/changeset/create";
        let body: String = format!("<osm>{changeset}</osm>");

        let res = self.put(url, body.as_str()).await?;

        match res.parse::<u64>() {
            Ok(id) => {
                info!("Created OSM changeset {id}");
                Ok(id)
            }

            Err(e) => {
                error!("Error parsing OSM changeset ID: {e:?}");
                Err(Error::OsmExchange)
            }
        }
    }

    pub async fn close_changeset(&self, id: u64) -> Result<()> {
        let url = format!("https://api.openstreetmap.org/api/0.6/changeset/{id}/close");
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
                error!("Error parsing OSM node id: {e:?}");
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
        let client_id = self.client_id.as_ref().ok_or_else(|| {
            error!("OSM client id not set.");
            Error::EnvNotSet
        })?;

        let redirect_uri = self.redirect_uri.as_ref().ok_or_else(|| {
            error!("OSM redirect URI not set.");
            Error::EnvNotSet
        })?;

        let secret = self.osm_client_secret.as_ref().ok_or_else(|| {
            error!("OSM Client Secret not set.");
            Error::EnvNotSet
        })?;

        let json = self
            .request_json(&format!(
                "https://api.openstreetmap.org/oauth/token?grant_type=authorization_code&code={code}&client_id={client_id}&client_secret={secret}&redirect_uri={redirect_uri}",
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
                    error!("Error parsing OSM element: {e:?}");
                }
            };
        }

        Ok(res)
    }

    pub async fn get_node(&self, id: u64) -> Result<Option<OsmElement>> {
        let url = format!("https://api.openstreetmap.org/api/0.6/node/{id}.json");

        let response = match self.client.get(url.to_string()).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error querying OSM API: {e}; URL: {url}");
                return Err(Error::OsmExchange);
            }
        };

        if response.status() == 410 {
            debug!("OSM node {id} is gone.");
            return Ok(None);
        }

        if response.status() == 404 {
            debug!("OSM node {id} not found.");
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
                error!("Error parsing OSM API response JSON: {e:?}");
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
                    error!("Error parsing OSM element: {e:?}");
                }
            };
        }

        Ok(None)
    }

    async fn put(&self, url: &str, body: &str) -> Result<String> {
        debug!("OSM PUT: {url}; body: {body}");

        let response = match self
            .client
            .put(url)
            .header("Authorization", format!("bearer {}", self.get_api_token()?))
            .body(body.to_string())
            .send()
            .await
        {
            Ok(response) => response,

            Err(e) => {
                error!("Error querying OSM API: {e}");
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
            error!("Error parsing OSM API response text: {e:?}");
            Error::OsmExchange
        })
    }

    async fn request_json(&self, url: &str) -> Result<Value> {
        let response = match self.client.get(url).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error querying OSM API: {e}");
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
                error!("Error parsing OSM API response JSON: {e:?}");
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
                xml.push_str(&format!("<tag k=\"height\" v=\"{value}\" />"));
            }
        }

        if let Some(value) = tree.circumference {
            if value > 0.0 {
                xml.push_str(&format!("<tag k=\"circumference\" v=\"{value}\" />"));
            }
        }

        if let Some(value) = tree.diameter {
            if value > 0.0 {
                xml.push_str(&format!("<tag k=\"diameter_crown\" v=\"{value}\" />"));
            }
        }

        // https://wiki.openstreetmap.org/wiki/Key:image
        let url = format!("https://yerevan.treemaps.app/tree/{}", tree.id);
        xml.push_str(&format!("<tag k=\"image\" v=\"{url}\" />"));

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

    fn format_changeset_comment(&self, comment: &str) -> String {
        let mut comment = comment.to_string();

        if let Some(hashtag) = &self.hashtag {
            comment.push_str(&format!("\n\n #{hashtag}"));
        }

        if let Some(activity) = &self.activity {
            comment.push_str(&format!("\n\n{activity}"));
        }

        comment
    }

    fn get_api_token(&self) -> Result<String> {
        let token = self.osm_token.as_ref().ok_or_else(|| {
            error!("OSM API token not set.");
            Error::Config("OSM token not set".to_string())
        })?;

        Ok(token.clone())
    }
}

impl Locatable for OsmClient {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;
        let secrets = locator.get::<Secrets>()?;

        let osm_client_secret = secrets.osm_client_secret.clone();

        let osm_token = secrets.osm_client_secret.clone();

        Ok(Self {
            client: reqwest::Client::new(),
            client_id: config.osm_client_id.clone(),
            redirect_uri: config.osm_redirect_uri.clone(),
            hashtag: config.osm_hashtag.clone(),
            activity: config.osm_activity.clone(),
            osm_client_secret,
            osm_token,
        })
    }
}
