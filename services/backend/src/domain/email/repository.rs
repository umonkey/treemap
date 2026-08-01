use super::models::Email;
use crate::infra::database::{Database, InsertQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use std::sync::Arc;

const TABLE: &str = "emails";

pub struct EmailRepository {
    db: Arc<Database>,
}

impl EmailRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn add(&self, email: &Email) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(email.to_attributes());
        self.db.add_record(query).await
    }

    pub async fn get_pending_or_failed(&self) -> Result<Vec<Email>> {
        let sql = format!(
            "SELECT * FROM `{}` WHERE `status` = 'PENDING' OR (`status` = 'FAILED' AND `attempts` < 5) ORDER BY `created_at` ASC",
            TABLE
        );
        self.fetch(&sql, &[]).await
    }

    pub async fn update(&self, email: &Email) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_value("status", Value::from(email.status.clone()))
            .with_value("attempts", Value::from(email.attempts as i64))
            .with_value(
                "last_error",
                match &email.last_error {
                    Some(v) => Value::from(v.clone()),
                    None => Value::Null,
                },
            )
            .with_value(
                "sent_at",
                match email.sent_at {
                    Some(v) => Value::from(v as i64),
                    None => Value::Null,
                },
            )
            .with_condition("id", Value::from(email.id as i64));
        self.db.update(query).await?;
        Ok(())
    }

    async fn fetch(&self, sql: &str, params: &[Value]) -> Result<Vec<Email>> {
        let rows = self.db.fetch_sql(sql, params).await?;
        rows.iter().map(Email::from_attributes).collect()
    }
}

impl Injectable for EmailRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
