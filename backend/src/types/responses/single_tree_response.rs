use crate::domain::user::User;
use crate::types::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SingleTreeResponse {
    pub id: String,
    pub lat: f64,
    pub lon: f64,
    pub osm_id: Option<u64>,
    pub species: String,
    pub notes: Option<String>,
    pub height: Option<f64>,
    pub circumference: Option<f64>,
    pub diameter: Option<f64>,
    pub state: String,
    pub added_at: u64,
    pub updated_at: u64,
    pub added_by: String,
    pub address: Option<String>,
    pub thumbnail_id: Option<String>,
    pub year: Option<i64>,
    pub like_count: i64,
    pub comment_count: u64,
    pub files: Vec<PublicFileInfo>,
    pub users: Vec<UserRead>,
    pub replaces: Option<String>,
    pub replaced_by: Option<String>,
}

impl SingleTreeResponse {
    pub fn from_tree(
        tree: &TreeRecord,
        files: &[FileRecord],
        users: &[User],
    ) -> SingleTreeResponse {
        let thumbnail_id = tree.thumbnail_id.map(|value| value.to_string());

        let users = users.iter().map(UserRead::from).collect();

        SingleTreeResponse {
            id: tree.id.to_string(),
            lat: tree.lat,
            lon: tree.lon,
            osm_id: tree.osm_id,
            species: tree.species.clone(),
            notes: tree.notes.clone(),
            height: tree.height,
            circumference: tree.circumference,
            diameter: tree.diameter,
            state: tree.state.clone(),
            added_at: tree.added_at,
            updated_at: tree.updated_at,
            added_by: tree.added_by.to_string(),
            address: tree.address.clone(),
            thumbnail_id,
            year: tree.year,
            like_count: tree.like_count,
            comment_count: tree.comment_count,
            replaces: Self::itos(tree.replaces),
            replaced_by: Self::itos(tree.replaced_by),
            files: files.iter().map(PublicFileInfo::from_file).collect(),
            users,
        }
    }

    fn itos(value: Option<u64>) -> Option<String> {
        value.map(|v| v.to_string())
    }
}
