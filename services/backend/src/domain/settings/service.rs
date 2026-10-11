use super::schemas::*;
use crate::domain::instance::InstanceService;
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

pub struct AppSettingsService {
    instances: Arc<InstanceService>,
}

impl AppSettingsService {
    pub async fn get(&self, domain: &str) -> Result<Settings> {
        let instance = self.instances.validate_instance(domain).await?;

        Ok(Settings {
            name: instance.name.clone(),
            description: instance.description.clone(),
            bbox: instance.bbox(),
        })
    }
}

impl Injectable for AppSettingsService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            instances: Arc::new(ctx.build::<InstanceService>()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::database::Value;
    use crate::services::{AppState, ContextExt};

    async fn setup() -> (AppState, Arc<AppSettingsService>) {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        state
            .database()
            .execute_sql(
                "INSERT INTO instances (domain, name, description, email, enabled, bbox_w, bbox_s, bbox_e, bbox_n) VALUES (?, ?, ?, ?, 1, ?, ?, ?, ?)",
                &[
                    Value::from("example.com".to_string()),
                    Value::from("Example Trees".to_string()),
                    Value::from("A test instance.".to_string()),
                    Value::from("test@example.com".to_string()),
                    Value::from(44.5_f64),
                    Value::from(40.1_f64),
                    Value::from(44.6_f64),
                    Value::from(40.2_f64),
                ],
            )
            .await
            .expect("Error inserting an instance.");

        let service = Arc::new(
            state
                .build::<AppSettingsService>()
                .expect("Error creating AppSettingsService"),
        );

        (state, service)
    }

    #[tokio::test]
    async fn test_get_settings() {
        let (_state, service) = setup().await;

        let settings = service
            .get("example.com")
            .await
            .expect("Error getting settings.");

        assert_eq!(settings.name, "Example Trees");
        assert_eq!(settings.description.as_deref(), Some("A test instance."));
        assert_eq!(settings.bbox, Some([44.5, 40.1, 44.6, 40.2]));
    }

    #[tokio::test]
    async fn test_get_unknown_instance() {
        let (_state, service) = setup().await;

        let result = service.get("unknown.example.com").await;

        assert!(matches!(result, Err(Error::InstanceNotFound)));
    }

    #[tokio::test]
    async fn test_get_settings_without_bbox() {
        let (state, service) = setup().await;

        state
            .database()
            .execute_sql(
                "INSERT INTO instances (domain, name, description, email, enabled) VALUES (?, ?, ?, ?, 1)",
                &[
                    Value::from("nobbox.example.com".to_string()),
                    Value::from("No Bbox".to_string()),
                    Value::from("An instance without bounds.".to_string()),
                    Value::from("test@example.com".to_string()),
                ],
            )
            .await
            .expect("Error inserting an instance.");

        let settings = service
            .get("nobbox.example.com")
            .await
            .expect("Error getting settings.");

        assert_eq!(settings.name, "No Bbox");
        assert_eq!(settings.bbox, None);
    }
}
