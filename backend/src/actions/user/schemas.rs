use crate::domain::user::User;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct UserList {
    pub users: Vec<UserRead>,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserRead {
    pub id: String,
    pub name: String,
    pub picture: String,
    pub trees_count: i64,
    pub comments_count: i64,
    pub updates_count: i64,
    pub files_count: i64,
}

impl From<Vec<User>> for UserList {
    fn from(users: Vec<User>) -> Self {
        let records = users.iter().map(UserRead::from).collect();
        Self { users: records }
    }
}

impl From<User> for UserRead {
    fn from(record: User) -> Self {
        UserRead {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
            trees_count: record.trees_count,
            comments_count: record.comments_count,
            updates_count: record.updates_count,
            files_count: record.files_count,
        }
    }
}

impl From<&User> for UserRead {
    fn from(record: &User) -> Self {
        UserRead {
            id: record.id.to_string(),
            name: record.name.clone(),
            picture: record.picture.clone(),
            trees_count: record.trees_count,
            comments_count: record.comments_count,
            updates_count: record.updates_count,
            files_count: record.files_count,
        }
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

        let read: UserRead = user.into();

        assert_eq!(read.id, "12345");
    }
}
