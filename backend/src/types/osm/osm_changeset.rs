use crate::utils::osm_tag;
use std::fmt;

pub struct OsmChangeset {
    pub created_by: String,
    pub host: String,
    pub bot: bool,
    pub source: String,
    pub comment: String,
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
