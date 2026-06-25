use crate::actions::user::UserRead;
use crate::domain::iam::ActorRights;
use crate::domain::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MeResponse {
    pub user: UserRead,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

impl MeResponse {
    pub fn from_domain(user: User, rights: ActorRights) -> Self {
        let mut permissions: Vec<String> = rights.permissions.into_iter().collect();
        permissions.sort();

        Self {
            user: UserRead::from_domain(user),
            roles: rights.roles,
            permissions,
        }
    }
}
