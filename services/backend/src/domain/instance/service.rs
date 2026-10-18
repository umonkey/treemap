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

    pub async fn list_for_osm_pull(&self) -> Result<Vec<Instance>> {
        self.instances.get_enabled_with_osm_pull().await
    }

    pub async fn update(&self, user_id: u64, domain: &str, req: InstanceUpdate) -> Result<()> {
        let instance = self.get(domain).await?;

        let name = req.name.unwrap_or_else(|| instance.name.clone());
        let description = req.description.or_else(|| instance.description.clone());
        let osm_pull = req.osm_pull.unwrap_or(instance.osm_pull);

        let osm_bot_id = match req.osm_bot_id {
            None => instance.osm_bot_id,

            Some(value) => {
                let value = value.trim();

                if value.is_empty() || value == "0" {
                    None
                } else {
                    match value.parse::<u64>() {
                        Ok(id) => Some(id),
                        Err(_) => {
                            return Err(Error::BadRequestMessage(
                                "OSM bot id must be a positive integer.".to_string(),
                            ));
                        }
                    }
                }
            }
        };

        if name.trim().is_empty() {
            return Err(Error::BadRequestMessage(
                "Instance name cannot be empty.".to_string(),
            ));
        }

        self.instances
            .update(
                instance.id,
                &name,
                description.as_deref(),
                osm_pull,
                osm_bot_id,
            )
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

        if osm_pull != instance.osm_pull {
            info!(
                "User {user_id} changed instance {} OSM pull: {} -> {}.",
                instance.domain, instance.osm_pull, osm_pull
            );
        }

        if osm_bot_id != instance.osm_bot_id {
            info!(
                "User {user_id} changed instance {} OSM bot id: {:?} -> {:?}.",
                instance.domain, instance.osm_bot_id, osm_bot_id
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
                    ..Default::default()
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
                    description: Some("Only description.".to_string()),
                    ..Default::default()
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
                    ..Default::default()
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
                    ..Default::default()
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

    #[tokio::test]
    async fn test_update_osm_settings() {
        let (_state, service) = setup().await;

        service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    osm_pull: Some(true),
                    osm_bot_id: Some("134022734140280832".to_string()),
                    ..Default::default()
                },
            )
            .await
            .expect("Error updating instance.");

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert!(instance.osm_pull);
        assert_eq!(instance.osm_bot_id, Some(134022734140280832));
    }

    #[tokio::test]
    async fn test_update_rejects_invalid_osm_bot_id() {
        let (_state, service) = setup().await;

        let result = service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    osm_pull: Some(true),
                    osm_bot_id: Some("not-a-number".to_string()),
                    ..Default::default()
                },
            )
            .await;

        assert!(matches!(result, Err(Error::BadRequestMessage(_))));

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert!(!instance.osm_pull);
        assert_eq!(instance.osm_bot_id, None);
    }

    #[tokio::test]
    async fn test_update_clears_osm_bot_id() {
        let (_state, service) = setup().await;

        service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    osm_bot_id: Some("42".to_string()),
                    ..Default::default()
                },
            )
            .await
            .expect("Error updating instance.");

        service
            .update(
                42,
                "example.com",
                InstanceUpdate {
                    osm_bot_id: Some("   ".to_string()),
                    ..Default::default()
                },
            )
            .await
            .expect("Error clearing the OSM bot id.");

        let instance = service
            .get("example.com")
            .await
            .expect("Error getting instance.");

        assert_eq!(instance.osm_bot_id, None);
    }

    #[tokio::test]
    async fn test_list_for_osm_pull() {
        let (state, service) = setup().await;

        state
            .database()
            .execute_sql(
                "INSERT INTO instances (domain, name, description, email, enabled, osm_pull) VALUES \
                 ('enabled.example.com', 'Enabled', NULL, 'enabled@example.com', 1, 1), \
                 ('disabled.example.com', 'Disabled', NULL, 'disabled@example.com', 0, 1), \
                 ('no-pull.example.com', 'No Pull', NULL, 'nopull@example.com', 1, 0)",
                &[],
            )
            .await
            .expect("Error inserting instances.");

        let instances = service
            .list_for_osm_pull()
            .await
            .expect("Error listing instances for OSM pull.");

        assert_eq!(instances.len(), 1);
        assert_eq!(instances[0].domain, "enabled.example.com");
    }
}
