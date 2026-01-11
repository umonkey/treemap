use crate::utils::osm_tag;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

pub struct OsmChangeset {
    pub created_by: String,
    pub host: String,
    pub bot: bool,
    pub source: String,
    pub comment: String,
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct OsmElement {
    pub r#type: String,
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
    pub timestamp: String,
    pub version: u64,
    pub changeset: u64,
    pub user: String,
    pub uid: u64,
    pub tags: HashMap<String, String>,
}

impl OsmChangeset {
    fn to_xml(&self) -> String {
        let mut body: String = "<changeset>".to_string();

        body.push_str(&osm_tag("created_by", &self.created_by));

        body.push_str(&osm_tag("host", &self.host));

        if self.bot {
            body.push_str(&osm_tag("bot", "yes"));
        }

        body.push_str(&osm_tag("source", &self.source));
        body.push_str(&osm_tag("comment", &self.comment));
        body.push_str("</changeset>");

        body
    }
}

impl fmt::Display for OsmChangeset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_xml())
    }
}
