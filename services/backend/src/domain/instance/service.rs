use super::models::Instance;
use super::repository::InstanceRepository;
use super::schemas::InstanceUpdate;
use crate::services::{Context, Injectable};
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct InstanceService {
    instances: Arc<InstanceRepository>,
}

impl InstanceService {
    pub async fn validate_instance(&self, domain: &str) -> Result<Instance> {
        let instance = self
            .instances
            .get_by_domain(domain)
            .await?
            .ok_or(Error::InstanceNotFound)?;

        if !instance.enabled {
            return Err(Error::InstanceDisabled);
        }

        Ok(instance)
    }

    pub async fn get(&self, domain: &str) -> Result<Instance> {
        self.instances
            .get_by_domain(domain)
            .await?
            .ok_or(Error::InstanceNotFound)
    }

    pub async fn update(&self, user_id: u64, domain: &str, req: InstanceUpdate) -> Result<()> {
        let instance = self.get(domain).await?;

        let name = req.name.unwrap_or_else(|| instance.name.clone());
        let description = req.description.or_else(|| instance.description.clone());

        if name.trim().is_empty() {
            return Err(Error::BadRequestMessage(
                "Instance name cannot be empty.".to_string(),
            ));
        }

        self.instances
            .update_profile(instance.id, &name, description.as_deref())
            .await?;

        if name != instance.name {
            info!(
                "User {user_id} changed instance {} name: '{}' -> '{}'.",
                instance.domain, instance.name, name
            );
        }

        if description != instance.description {
            info!(
                "User {user_id} changed instance {} description: {:?} -> {:?}.",
                instance.domain, instance.description, description
            );
        }

        Ok(())
    }
}

impl Injectable for InstanceService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            instances: Arc::new(ctx.build::<InstanceRepository>()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::database::Value;
    use crate::services::{AppState, ContextExt};

    async fn setup() -> (AppState, Arc<InstanceService>) {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        state
            .database()
            .execute_sql(
                "INSERT INTO instances (domain, name, description, email, enabled) VALUES (?, ?, ?, ?, 1)",
                &[
                    Value::from("example.com".to_string()),
                    Value::from("Example Trees".to_string()),
                    Value::from("A test instance.".to_string()),
                    Value::from("test@example.com".to_string()),
                ],
            )
            .await
            .expect("Error inserting an instance.");

        let service = Arc::new(
            state
                .build::<InstanceService>()
                .expect("Error creating InstanceService"),
        );

        (state, service)
    }

    #[tokio::test]
    async fn test_get() {
        let (_state, service) = setup().await;

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert_eq!(instance.name, "Example Trees");
        assert_eq!(instance.description.as_deref(), Some("A test instance."));
    }

    #[tokio::test]
    async fn test_get_unknown_instance() {
        let (_state, service) = setup().await;

        let result = service.get("unknown.example.com").await;

        assert!(matches!(result, Err(Error::InstanceNotFound)));
    }

    #[tokio::test]
    async fn test_update_both_fields() {
        let (_state, service) = setup().await;

        service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    name: Some("New Name".to_string()),
                    description: Some("New description.".to_string()),
                },
            )
            .await
            .expect("Error updating instance.");

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert_eq!(instance.name, "New Name");
        assert_eq!(instance.description.as_deref(), Some("New description."));
    }

    #[tokio::test]
    async fn test_update_partial_fields() {
        let (_state, service) = setup().await;

        service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    name: None,
                    description: Some("Only description.".to_string()),
                },
            )
            .await
            .expect("Error updating instance.");

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert_eq!(instance.name, "Example Trees");
        assert_eq!(instance.description.as_deref(), Some("Only description."));
    }

    #[tokio::test]
    async fn test_update_unknown_instance() {
        let (_state, service) = setup().await;

        let result = service
            .update(
                42,
                "unknown.example.com",
                InstanceUpdate {
                    name: Some("New Name".to_string()),
                    description: None,
                },
            )
            .await;

        assert!(matches!(result, Err(Error::InstanceNotFound)));
    }

    #[tokio::test]
    async fn test_update_rejects_empty_name() {
        let (_state, service) = setup().await;

        let result = service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    name: Some("   ".to_string()),
                    description: None,
                },
            )
            .await;

        assert!(matches!(result, Err(Error::BadRequestMessage(_))));

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert_eq!(instance.name, "Example Trees");
    }
}
