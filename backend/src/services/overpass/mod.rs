mod node_parser;

use log::{debug, error};
use serde_json::Value;
use url::Url;

use crate::types::{Error, OsmTreeRecord, Result};
use crate::utils::{get_overpass_endpoint, get_overpass_query};

use self::node_parser::NodeParser;

pub struct OverpassClient {
    client: reqwest::Client,
    endpoint: String,
    query: String,
}

impl OverpassClient {
    pub fn init() -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            endpoint: get_overpass_endpoint(),
            query: get_overpass_query(),
        }
    }

    pub async fn query(&self) -> Result<Vec<OsmTreeRecord>> {
        let query_url = self.get_query_url()?;
        let json = self.read_json(&query_url).await?;

        let elements = match json["elements"].as_array() {
            Some(elements) => elements,

            None => {
                error!("OSM response does not contain elements array.");
                return Err(Error::OsmExchange);
            }
        };

        let mut trees: Vec<OsmTreeRecord> = Vec::new();

        for element in elements {
            match NodeParser::parse(element) {
                Some(tree) => trees.push(tree),
                None => {
                    error!("Error parsing OSM node: {:?}", element);
                }
            }
        }

        Ok(trees)
    }

    async fn read_json(&self, url: &str) -> Result<Value> {
        debug!("Sending Overpass query to: {}", url);

        let response = match self.client.get(url).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error sending Overpass query: {}", e);
                return Err(Error::OsmExchange);
            }
        };

        if response.status() != 200 {
            error!("Overpass query failed with status: {}", response.status());
            return Err(Error::OsmExchange);
        }

        let json: Value = match response.json().await {
            Ok(json) => json,

            Err(e) => {
                error!("Error parsing Overpass response JSON: {:?}", e);
                return Err(Error::OsmExchange);
            }
        };

        Ok(json)
    }

    fn get_query_url(&self) -> Result<String> {
        match Url::parse_with_params(&self.endpoint, &[("data", self.query.clone())]) {
            Ok(url) => Ok(url.to_string()),

            Err(e) => {
                error!("Error building Overpass query URL: {}", e);
                Err(Error::OsmExchange)
            }
        }
    }
}
