//! This is how a single species is stored in the database.

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Species {
    pub name: String,
    pub local: String,
    pub keywords: String,
    pub wikidata_id: String,
}
