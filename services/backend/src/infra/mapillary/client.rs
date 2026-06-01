use super::schemas::MapillaryResponse;
use crate::services::{Context, Injectable};
use crate::types::*;
use log::debug;
use reqwest::Client;

pub struct MapillaryClient {
    client: Client,
    token: Option<String>,
    org_id: Option<String>,
}

impl MapillaryClient {
    pub async fn fetch_panoramas(&self, limit: u32) -> Result<MapillaryResponse> {
        let token = self
            .token
            .as_ref()
            .ok_or_else(|| Error::Config("MAPILLARY_TOKEN not set".to_string()))?;

        let org_id = self
            .org_id
            .as_ref()
            .ok_or_else(|| Error::Config("MAPILLARY_ORG_ID not set".to_string()))?;

        let url = format!(
            "https://graph.mapillary.com/images?access_token={}&organization_id={}&is_pano=true&fields=id,sequence,captured_at,is_pano,quality_score,geometry,compass_angle,computed_geometry,computed_compass_angle&limit={}",
            token, org_id, limit
        );

        self.fetch_url(&url).await
    }

    pub async fn fetch_image(&self, id: &str) -> Result<super::schemas::MapillaryImage> {
        let token = self
            .token
            .as_ref()
            .ok_or_else(|| Error::Config("MAPILLARY_TOKEN not set".to_string()))?;

        let url = format!(
            "https://graph.mapillary.com/{}?access_token={}&fields=id,sequence,captured_at,is_pano,geometry,compass_angle,computed_geometry,computed_compass_angle,thumb_2048_url",
            id, token
        );

        debug!("Fetching Mapillary Image: {}", id);

        let response = self.client.get(&url).send().await.map_err(|e| {
            Error::MapillaryExchange(format!("Error querying Mapillary API: {}", e))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::MapillaryExchange(format!(
                "Mapillary API returned {}: {}",
                status, text
            )));
        }

        let data: super::schemas::MapillaryImage = response.json().await.map_err(|e| {
            Error::MapillaryExchange(format!("Error parsing Mapillary response: {}", e))
        })?;

        Ok(data)
    }

    pub async fn fetch_url(&self, url: &str) -> Result<MapillaryResponse> {
        debug!("Fetching Mapillary URL: {}", url);

        let response = self.client.get(url).send().await.map_err(|e| {
            Error::MapillaryExchange(format!("Error querying Mapillary API: {}", e))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(Error::MapillaryExchange(format!(
                "Mapillary API returned {}: {}",
                status, text
            )));
        }

        let data: MapillaryResponse = response.json().await.map_err(|e| {
            Error::MapillaryExchange(format!("Error parsing Mapillary response: {}", e))
        })?;

        Ok(data)
    }
}

impl Injectable for MapillaryClient {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let secrets = ctx.secrets();

        Ok(Self {
            client: reqwest::Client::new(),
            token: secrets.mapillary_token.clone(),
            org_id: secrets.mapillary_org_id.clone(),
        })
    }
}
