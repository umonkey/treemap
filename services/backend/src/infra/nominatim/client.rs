use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use log::{debug, error, info};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

const ACCEPT_LANGUAGE: &str = "en-US,en;q=0.5";

#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    road: Option<String>,
    house_number: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResponsePayload {
    address: AddressInfo,
}

pub struct NominatimClient {
    http: reqwest::Client,
    user_agent: String,
    referrer: String,
}

impl NominatimClient {
    pub fn new(user_agent: String, referrer: String) -> Self {
        let http = reqwest::Client::new();
        Self {
            http,
            user_agent,
            referrer,
        }
    }

    pub async fn get_address(
        &self,
        lat: f64,
        lon: f64,
        include_building: bool,
    ) -> Result<Option<String>> {
        // NB! Building numbers are only returned at zoom>=18.  But with zoom=18 Nominatim
        // often takes street names from the closest bigger building which can have address
        // from the adjacent street.  With zoom=16 we get much better street-only results, so
        // we only raise the zoom when a building number is explicitly requested.
        let zoom = if include_building { 18 } else { 16 };
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?format=json&lat={lat}&lon={lon}&zoom={zoom}&addressdetails=1"
        );

        debug!("Requesting address from Nominatim: {url}");

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

        let address = match (json.address.road, json.address.house_number) {
            (Some(road), Some(number)) if include_building => format!("{road} {number}"),
            (Some(road), _) => road,
            (None, _) => {
                info!("Could not resolve {lat},{lon} to an address.");
                return Ok(None);
            }
        };

        info!("Resolved {lat},{lon} as: {address}");
        Ok(Some(address))
    }
}

impl Default for NominatimClient {
    fn default() -> Self {
        Self::new(
            "TreeMap".to_string(),
            "https://github.com/umonkey/treemap/".to_string(),
        )
    }
}

impl Injectable for NominatimClient {
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
