//! Contains data for the Street report.

use serde::Serialize;

#[derive(Serialize)]
pub struct StreetReport {
    pub street: String,
    pub total: usize,
    pub area: u64,
    pub states: Vec<TreesByStateReport>,
    pub heights: Vec<TreesByHeightReport>,
    pub crowns: Vec<TreesByCrownReport>,
    pub girths: Vec<TreesByGirthReport>,
    pub species: Vec<TreesBySpeciesReport>,
}

#[derive(Serialize)]
pub struct TreesByCrownReport {
    pub value: u64,
    pub count: usize,
}

#[derive(Serialize)]
pub struct TreesByGirthReport {
    pub value: u64,
    pub count: usize,
}

#[derive(Serialize)]
pub struct TreesByHeightReport {
    pub value: u64,
    pub count: usize,
}

#[derive(Default, Serialize)]
pub struct TreesBySpeciesReport {
    pub species: String,
    pub count: usize,
    pub height: f64,
    pub diameter: f64,
    pub girth: f64,
}

#[derive(Serialize)]
pub struct TreesByStateReport {
    pub state: String,
    pub count: usize,
}
