//! This structure wraps a list of property changes for a tree.
//! Changes can be made by different users, so the list of users
//! is added to pre-populate the cache.

use crate::actions::user::UserRead;
use crate::domain::prop::PropRecord;
use crate::domain::user::User;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PropList {
    pub props: Vec<PropResponse>,
    pub users: Vec<UserRead>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PropResponse {
    pub id: String,
    pub added_at: u64,
    pub added_by: String,
    pub name: String,
    pub value: String,
}

impl PropList {
    pub fn from_records(props: &[PropRecord], users: &[User]) -> PropList {
        let props: Vec<PropResponse> = props.iter().map(PropResponse::from).collect();
        let users: Vec<UserRead> = users.iter().map(UserRead::from).collect();

        PropList { props, users }
    }
}

impl From<PropRecord> for PropResponse {
    fn from(record: PropRecord) -> Self {
        PropResponse {
            id: record.id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.to_string(),
            name: record.name.clone(),
            value: record.value.clone(),
        }
    }
}

impl From<&PropRecord> for PropResponse {
    fn from(record: &PropRecord) -> Self {
        PropResponse {
            id: record.id.to_string(),
            added_at: record.added_at,
            added_by: record.added_by.to_string(),
            name: record.name.clone(),
            value: record.value.clone(),
        }
    }
}
