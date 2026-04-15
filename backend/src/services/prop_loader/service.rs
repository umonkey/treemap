use super::schemas::PropList;
use crate::domain::prop::PropRecord;
use crate::domain::user::UserRepository;
use crate::services::{Context, ContextExt, Injectable};
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

impl Injectable for PropLoader {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let locator = ctx.locator();
        Ok(Self {
            users: Arc::new(locator.build::<UserRepository>()?),
        })
    }
}
