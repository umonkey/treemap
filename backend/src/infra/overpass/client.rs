use crate::domain::osm::OsmTreeRecord;
use crate::infra::config::Config;
use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use log::{debug, error, warn};
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

pub struct OverpassClient {
    client: reqwest::Client,
    endpoint: String,
    query: String,
}

impl OverpassClient {
    pub fn new(endpoint: String, query: String) -> Self {
        let client = reqwest::Client::new();

        Self {
            client,
            endpoint,
            query,
        }
    }

    pub async fn query(&self) -> Result<Vec<OsmTreeRecord>> {
        let query_url = self.get_query_url()?;
        let json = self.read_json(&query_url).await?;

        let elements = match json["elements"].as_array() {
            Some(elements) => elements,

            None => {
                return Err(Error::OsmExchange(
                    "OSM response does not contain elements array".to_string(),
                ));
            }
        };

        let mut trees: Vec<OsmTreeRecord> = Vec::new();

        for element in elements {
            match OsmTreeRecord::from_overpass(element) {
                Some(tree) => trees.push(tree),
                None => {
                    error!("Error parsing OSM node: {element:?}");
                }
            }
        }

        Ok(trees)
    }

    async fn read_json(&self, url: &str) -> Result<Value> {
        let mut attempt = 0;
        let max_attempts = 10;

        loop {
            attempt += 1;
            debug!("Sending Overpass query to: {url} (attempt {attempt})");

            let response = match self.client.get(url).send().await {
                Ok(response) => response,

                Err(e) => {
                    if attempt >= max_attempts {
                        return Err(Error::OsmExchange(format!(
                            "Error sending Overpass query: {e}"
                        )));
                    }
                    warn!("Error sending Overpass query (attempt {attempt}): {e}. Retrying...");
                    sleep(Duration::from_secs(10 * attempt as u64)).await;
                    continue;
                }
            };

            let status = response.status();
            if status.is_success() {
                let json: Value = match response.json().await {
                    Ok(json) => json,

                    Err(e) => {
                        return Err(Error::OsmExchange(format!(
                            "Error parsing Overpass response JSON: {e:?}"
                        )));
                    }
                };

                return Ok(json);
            }

            if status.is_server_error() {
                if attempt >= max_attempts {
                    return Err(Error::OsmExchange(format!(
                        "Overpass query failed with status {status}"
                    )));
                }

                warn!(
                    "Overpass query failed with status {status} (attempt {attempt}). Retrying..."
                );
                sleep(Duration::from_secs(10 * attempt as u64)).await;
                continue;
            }

            return Err(Error::OsmExchange(format!(
                "Overpass query failed with status {status}"
            )));
        }
    }

    fn get_query_url(&self) -> Result<String> {
        match Url::parse_with_params(&self.endpoint, &[("data", self.query.clone())]) {
            Ok(url) => Ok(url.to_string()),

            Err(e) => Err(Error::OsmExchange(format!(
                "Error building Overpass query URL: {e}"
            ))),
        }
    }
}

impl Locatable for OverpassClient {
    fn create(locator: &Locator) -> Result<Self> {
        let config = locator.get::<Config>()?;

        Ok(Self::new(
            config.overpass_endpoint.clone(),
            config.overpass_query.clone(),
        ))
    }
}
