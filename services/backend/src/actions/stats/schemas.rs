use crate::domain::user::UserActivity;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TopUser {
    pub id: String,
    pub name: String,
    pub picture: String,
    pub count: u64,
}

#[derive(Debug, Serialize)]
pub struct TopUserList {
    pub users: Vec<TopUser>,
}

impl From<UserActivity> for TopUser {
    fn from(item: UserActivity) -> Self {
        Self {
            id: item.user.id.to_string(),
            name: item.user.name,
            picture: item.user.picture,
            count: item.count,
        }
    }
}

impl From<Vec<UserActivity>> for TopUserList {
    fn from(items: Vec<UserActivity>) -> Self {
        let users = items.into_iter().map(TopUser::from).collect();
        Self { users }
    }
}
