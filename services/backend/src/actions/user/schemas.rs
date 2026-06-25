use crate::domain::iam::ActorRights;
use crate::domain::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserRead {
    pub id: String,
    pub name: String,
    pub picture: String,
    pub email: String,
    pub trees_count: i64,
    pub comments_count: i64,
    pub updates_count: i64,
    pub files_count: i64,
}

#[derive(Debug, Serialize)]
pub struct UserReadWithRights {
    pub user: UserRead,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct UserList {
    pub users: Vec<UserReadWithRights>,
}

impl UserRead {
    pub fn from_domain(user: User) -> Self {
        UserRead {
            id: user.id.to_string(),
            name: user.name.clone(),
            picture: user.picture.clone(),
            email: user.email.clone(),
            trees_count: user.trees_count,
            comments_count: user.comments_count,
            updates_count: user.updates_count,
            files_count: user.files_count,
        }
    }
}

impl UserReadWithRights {
    pub fn from_domain(user: User, rights: ActorRights) -> Self {
        let mut permissions: Vec<String> = rights.permissions.into_iter().collect();
        permissions.sort();

        UserReadWithRights {
            user: UserRead::from_domain(user),
            roles: rights.roles,
            permissions,
        }
    }

    pub fn from_user(user: User) -> Self {
        UserReadWithRights {
            user: UserRead::from_domain(user),
            roles: Vec::new(),
            permissions: Vec::new(),
        }
    }
}

impl From<User> for UserRead {
    fn from(user: User) -> Self {
        Self::from_domain(user)
    }
}

impl From<&User> for UserRead {
    fn from(user: &User) -> Self {
        Self::from_domain(user.clone())
    }
}

impl From<Vec<User>> for UserList {
    fn from(users: Vec<User>) -> Self {
        let records = users
            .into_iter()
            .map(UserReadWithRights::from_user)
            .collect();
        Self { users: records }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_to_user_read() {
        let user = User {
            id: 12345,
            name: "Alice".to_string(),
            picture: "http://example.com/".to_string(),
            ..Default::default()
        };

        let read = UserRead::from(user);

        assert_eq!(read.id, "12345");
    }
}
