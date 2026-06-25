use super::models::ActorRights;
use super::repository::IamRepository;
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

pub struct IamService {
    repo: Arc<IamRepository>,
}

impl IamService {
    pub async fn get_user_rights(&self, user_id: u64) -> Result<ActorRights> {
        let roles = self.repo.get_user_roles(user_id).await?;
        let permissions = self.repo.get_user_permissions(user_id).await?;

        Ok(ActorRights { roles, permissions })
    }

    pub async fn require_permission(&self, user_id: u64, permission: &str) -> Result<()> {
        let rights = self.get_user_rights(user_id).await?;

        if rights.has_permission(permission) {
            Ok(())
        } else {
            Err(Error::AccessDenied)
        }
    }

    pub async fn assign_default_role(&self, user_id: u64) -> Result<()> {
        self.repo.assign_role(user_id, "user").await
    }
}

impl Injectable for IamService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            repo: Arc::new(ctx.build::<IamRepository>()?),
        })
    }
}
