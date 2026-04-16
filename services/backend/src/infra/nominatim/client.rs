use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

const USER_AGENT: &str = "TreeMap/1.0";
const REFERRER: &str = "https://github.com/umonkey/treemap/";
const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.5";

#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    road: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResponsePayload {
    address: AddressInfo,
}

pub struct NominatimClient {
    http: reqwest::Client,
}

impl NominatimClient {
    pub fn new() -> Self {
        let http = reqwest::Client::new();
        Self { http }
    }

    pub async fn get_street_address(&self, lat: f64, lon: f64) -> Result<Option<String>> {
        // NB! Use zoom=16 to avoid street confusion.  With zoom=18 it often takes street
        // names from the closes bigger building which can have address from the adjacent
        // street.  With zoom=16 we get much better results.
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?format=json&lat={lat}&lon={lon}&zoom=16&addressdetails=1"
        );

        debug!("Requesting address from Nominatim: {url}");

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static(USER_AGENT));
        headers.insert("Referer", HeaderValue::from_static(REFERRER));
        headers.insert("Accept-Language", HeaderValue::from_static(ACCEPT_LANGUAGE));

        let response = match self.http.get(&url).headers(headers).send().await {
            Ok(response) => response,

            Err(e) => {
                error!("Error contacting Nominatim: {e}");
                return Err(Error::AddressNotFound);
            }
        };

        if response.status() != 200 {
            error!("Nominatim query failed with status: {}", response.status());
            return Err(Error::AddressNotFound);
        }

        let json = match response.json::<ResponsePayload>().await {
            Ok(json) => json,

            Err(e) => {
                error!("Error parsing Nominatim response: {e:?}");
                return Err(Error::AddressNotFound);
            }
        };

        if let Some(value) = json.address.road {
            info!("Resolved {lat},{lon} as: {value}");
            Ok(Some(value))
        } else {
            info!("Could not resolve {lat},{lon} to an address.");
            Ok(None)
        }
    }
}

impl Default for NominatimClient {
    fn default() -> Self {
        Self::new()
    }
}

impl Injectable for NominatimClient {
    fn inject(_ctx: &dyn Context) -> Result<Self> {
        Ok(Self::new())
    }
}
