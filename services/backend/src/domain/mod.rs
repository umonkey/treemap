//! This module contains the code split by the business domain.
//! Each included module has its own actions, responders and domain code.

#[allow(dead_code)]
pub mod alert;
pub mod alert_file;
pub mod alert_photo;
pub mod comment;
pub mod email;
pub mod export;
pub mod file;
pub mod health;
pub mod heatmap;
pub mod iam;
pub mod instance;
pub mod like;
pub mod login;
pub mod observation;
pub mod osm;
pub mod panorama;
pub mod photo;
pub mod prop;
pub mod settings;
pub mod species;
pub mod stats;
pub mod storage;
pub mod street;
pub mod training;
#[allow(dead_code)]
pub mod tree;
pub mod tree_file;
#[allow(dead_code)]
pub mod upload;
pub mod user;
pub mod water;
pub mod water_file;
