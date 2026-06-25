use std::collections::HashSet;

pub struct ActorRights {
    pub roles: Vec<String>,
    pub permissions: HashSet<String>,
}

impl ActorRights {
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission) || self.roles.contains(&"admin".to_string())
    }
}
