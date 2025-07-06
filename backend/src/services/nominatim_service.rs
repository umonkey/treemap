use crate::services::*;
use crate::types::*;
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

pub struct NominatimService {
    http: reqwest::Client,
}

impl NominatimService {
    pub async fn get_street_address(&self, lat: f64, lon: f64) -> Result<Option<String>> {
        let url = format!(
            "https://nominatim.openstreetmap.org/reverse?format=json&lat={lat}&lon={lon}&zoom=18&addressdetails=1"
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

impl Locatable for NominatimService {
    fn create(_locator: &Locator) -> Result<Self> {
        let http = reqwest::Client::new();
        Ok(Self { http })
    }
}
