use crate::infra::database::{Database, InsertQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::collections::HashSet;
use std::sync::Arc;

pub struct IamRepository {
    db: Arc<Database>,
}

impl IamRepository {
    pub async fn get_user_roles(&self, user_id: u64) -> Result<Vec<String>> {
        let sql = "SELECT role_id FROM user_roles WHERE user_id = ?";
        let params = &[Value::from(user_id as i64)];
        let records = self.db.fetch_sql(sql, params).await?;

        records
            .iter()
            .map(|r| r.require_string("role_id"))
            .collect::<Result<Vec<String>>>()
    }

    pub async fn get_user_permissions(&self, user_id: u64) -> Result<HashSet<String>> {
        let sql = "
            SELECT DISTINCT permission_id
            FROM role_permissions
            WHERE role_id IN (SELECT role_id FROM user_roles WHERE user_id = ?)
        ";
        let params = &[Value::from(user_id as i64)];
        let records = self.db.fetch_sql(sql, params).await?;

        records
            .iter()
            .map(|r| r.require_string("permission_id"))
            .collect::<Result<HashSet<String>>>()
    }

    pub async fn assign_role(&self, user_id: u64, role_id: &str) -> Result<()> {
        let query = InsertQuery::new("user_roles")
            .with_value("user_id", Value::from(user_id as i64))
            .with_value("role_id", Value::from(role_id.to_string()));

        self.db.add_record(query).await
    }
}

impl Injectable for IamRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
