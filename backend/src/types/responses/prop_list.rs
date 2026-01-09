//! This structure wraps a list of property changes for a tree.
//! Changes can be made by different users, so the list of users
//! is added to pre-populate the cache.

use crate::types::PropRecord;
use crate::types::PropResponse;
use crate::types::User;
use crate::types::UserResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PropList {
    pub props: Vec<PropResponse>,
    pub users: Vec<UserResponse>,
}

impl PropList {
    pub fn from_records(props: &[PropRecord], users: &[User]) -> PropList {
        let props: Vec<PropResponse> = props.iter().map(PropResponse::from).collect();
        let users: Vec<UserResponse> = users.iter().map(UserResponse::from).collect();

        PropList { props, users }
    }
}
