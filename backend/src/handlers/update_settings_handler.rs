use crate::domain::user::UserRepository;
use crate::infra::queue::Queue;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateSettingsHandler {
    users: Arc<UserRepository>,
    queue: Arc<Queue>,
}

impl UpdateSettingsHandler {
    pub async fn handle(&self, req: UpdateSettingsRequest) -> Result<()> {
        let user = self
            .users
            .get(req.user_id)
            .await?
            .ok_or(Error::UserNotFound)?;

        self.users.update_name(req.user_id, &req.name).await?;
        info!("User {} display name changed to {}", user.id, req.name);

        if let Some(file_id) = &req.picture {
            let file_id = file_id.parse::<u64>().map_err(|_| Error::BadImage)?;

            let message = UpdateUserpicMessage {
                user_id: req.user_id,
                file_id,
            };

            self.queue.push(&message.encode()).await?;
        }

        Ok(())
    }
}

impl Locatable for UpdateSettingsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
            queue: locator.get::<Queue>()?,
        })
    }
}
