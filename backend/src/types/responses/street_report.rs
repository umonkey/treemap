//! Contains data for the Street report.

use crate::reports::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct StreetReport {
    pub street: String,
    pub total: usize,
    pub states: Vec<TreesByStateReport>,
    pub heights: Vec<TreesByHeightReport>,
    pub crowns: Vec<TreesByCrownReport>,
    pub griths: Vec<TreesByGrithReport>,
}
