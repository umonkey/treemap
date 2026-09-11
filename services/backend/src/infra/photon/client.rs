use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.5";

#[derive(Debug, Deserialize)]
pub struct FeatureProperties {
    street: Option<String>,
    housenumber: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    properties: FeatureProperties,
}

#[derive(Debug, Deserialize)]
pub struct ResponsePayload {
    features: Vec<Feature>,
}

pub struct PhotonClient {
    http: reqwest::Client,
    user_agent: String,
    referrer: String,
}

impl PhotonClient {
    pub fn new(user_agent: String, referrer: String) -> Self {
        let http = reqwest::Client::new();
        Self {
            http,
            user_agent,
            referrer,
        }
    }

    pub async fn get_address(&self, lat: f64, lon: f64) -> Result<Option<String>> {
        let url = format!("https://photon.komoot.io/reverse?lat={lat}&lon={lon}&limit=1&lang=en");

        debug!("Requesting address from Photon: {url}");

        let mut headers = HeaderMap::new();
        headers.insert(
            "User-Agent",
            HeaderValue::from_str(&self.user_agent).unwrap_or(HeaderValue::from_static("TreeMap")),
        );
        headers.insert(
            "Referer",
            HeaderValue::from_str(&self.referrer).unwrap_or(HeaderValue::from_static(
                "https://github.com/umonkey/treemap",
            )),
        );
        headers.insert("Accept-Language", HeaderValue::from_static(ACCEPT_LANGUAGE));

        let response = match self.http.get(&url).headers(headers).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error contacting Photon: {e}");
                return Err(Error::AddressNotFound);
            }
        };

        if response.status() != 200 {
            error!("Photon query failed with status: {}", response.status());
            return Err(Error::AddressNotFound);
        }

        let json = match response.json::<ResponsePayload>().await {
            Ok(json) => json,

            Err(e) => {
                error!("Error parsing Photon response: {e:?}");
                return Err(Error::AddressNotFound);
            }
        };

        let feature = match json.features.into_iter().next() {
            Some(feature) => feature,

            None => {
                info!("Could not resolve {lat},{lon} to a building address.");
                return Ok(None);
            }
        };

        let address = match (feature.properties.street, feature.properties.housenumber) {
            (Some(street), Some(number)) => format!("{street} {number}"),
            (Some(street), None) => street,
            (None, _) => {
                info!("Could not resolve {lat},{lon} to a building address.");
                return Ok(None);
            }
        };

        info!("Resolved {lat},{lon} via Photon as: {address}");
        Ok(Some(address))
    }
}

impl Default for PhotonClient {
    fn default() -> Self {
        Self::new(
            "TreeMap".to_string(),
            "https://github.com/umonkey/treemap/".to_string(),
        )
    }
}

impl Injectable for PhotonClient {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();
        let user_agent = format!(
            "TreeMap/{} ({})",
            env!("CARGO_PKG_VERSION"),
            config.app_contact
        );
        let referrer = config.app_contact.clone();

        Ok(Self::new(user_agent, referrer))
    }
}
