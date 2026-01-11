use super::schemas::PropList;
use crate::domain::prop::PropRecord;
use crate::domain::user::UserRepository;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use std::sync::Arc;

pub struct PropLoader {
    users: Arc<UserRepository>,
}

impl PropLoader {
    pub async fn load_list(&self, props: &[PropRecord]) -> Result<PropList> {
        let user_ids: Vec<u64> = props.iter().map(|r| r.added_by).collect();
        let users = self.users.get_multiple(&user_ids).await?;

        Ok(PropList::from_records(props, &users))
    }
}

impl Locatable for PropLoader {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
        })
    }
}
