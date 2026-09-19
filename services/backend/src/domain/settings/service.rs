use super::schemas::*;
use crate::domain::user::UserRepository;
use crate::infra::queue::Queue;
use crate::services::queue_consumer::UpdateUserpicMessage;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use log::info;
use std::sync::Arc;

pub struct SettingsService {
    users: Arc<UserRepository>,
    queue: Arc<Queue>,
}

impl SettingsService {
    pub async fn update(&self, req: UpdateSettingsRequest) -> Result<()> {
        let user = self
            .users
            .get(req.user_id)
            .await?
            .ok_or(Error::UserNotFound)?;

        let display_name = req.name.as_deref().unwrap_or(&user.name);

        if let Some(name) = &req.name {
            self.users.update_name(req.user_id, name).await?;
            info!("User {} ({}) updated their name.", user.id, display_name);
        }

        if let Some(file_id) = &req.picture {
            let file_id = file_id.parse::<u64>().map_err(|_| Error::BadImage)?;

            let message = UpdateUserpicMessage {
                user_id: req.user_id,
                file_id,
            };

            self.queue.push(&message.encode()).await?;
            info!("User {} ({}) updated their picture.", user.id, display_name);
        }

        Ok(())
    }
}

impl Injectable for SettingsService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            users: Arc::new(ctx.build::<UserRepository>()?),
            queue: ctx.queue(),
        })
    }
}
