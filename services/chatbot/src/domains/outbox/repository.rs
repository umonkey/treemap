use super::model::OutboxMessage;
use crate::infra::database::DatabaseClient;
use libsql::{params_from_iter, Value};
use std::sync::Arc;

pub struct OutboxRepository {
    db: Arc<DatabaseClient>,
}

impl OutboxRepository {
    pub fn new(db: Arc<DatabaseClient>) -> Self {
        Self { db }
    }

    pub async fn enqueue(&self, alert_id: i64, chat_id: i64, text: &str) -> anyhow::Result<i64> {
        let conn = self.db.connect().await?;
        let sql = "INSERT INTO chatbot_outbox (alert_id, chat_id, text, status, created_at, next_retry_at, attempts) 
                   VALUES (?, ?, ?, 'pending', unixepoch(), unixepoch(), 0)";
        let params = vec![
            Value::Integer(alert_id),
            Value::Integer(chat_id),
            Value::Text(text.to_string()),
        ];
        conn.execute(sql, params_from_iter(params)).await?;
        Ok(conn.last_insert_rowid())
    }

    pub async fn fetch_pending(&self, limit: i64) -> anyhow::Result<Vec<OutboxMessage>> {
        let conn = self.db.connect().await?;
        conn.execute("BEGIN IMMEDIATE", params_from_iter(Vec::<Value>::new()))
            .await?;

        let sql = "SELECT id, alert_id, chat_id, text, status, created_at, sent_at, next_retry_at, attempts, error_message 
                   FROM chatbot_outbox 
                   WHERE status = 'pending' AND next_retry_at <= unixepoch() 
                   ORDER BY created_at ASC 
                   LIMIT ?";
        let mut stmt = match conn.prepare(sql).await {
            Ok(s) => s,
            Err(e) => {
                let _ = conn
                    .execute("ROLLBACK", params_from_iter(Vec::<Value>::new()))
                    .await;
                return Err(e.into());
            }
        };
        let mut rows = match stmt
            .query(params_from_iter(vec![Value::Integer(limit)]))
            .await
        {
            Ok(r) => r,
            Err(e) => {
                let _ = conn
                    .execute("ROLLBACK", params_from_iter(Vec::<Value>::new()))
                    .await;
                return Err(e.into());
            }
        };
        let mut messages = Vec::new();

        loop {
            match rows.next().await {
                Ok(Some(row)) => {
                    match (|| -> anyhow::Result<OutboxMessage> {
                        Ok(OutboxMessage {
                            id: row.get(0)?,
                            alert_id: row.get(1)?,
                            chat_id: row.get(2)?,
                            text: row.get(3)?,
                            status: row.get(4)?,
                            created_at: row.get(5)?,
                            sent_at: row.get(6)?,
                            next_retry_at: row.get(7)?,
                            attempts: row.get(8)?,
                            error_message: row.get(9)?,
                        })
                    })() {
                        Ok(msg) => messages.push(msg),
                        Err(e) => {
                            let _ = conn
                                .execute("ROLLBACK", params_from_iter(Vec::<Value>::new()))
                                .await;
                            return Err(e);
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    let _ = conn
                        .execute("ROLLBACK", params_from_iter(Vec::<Value>::new()))
                        .await;
                    return Err(e.into());
                }
            }
        }

        for msg in &messages {
            if let Err(e) = conn
                .execute(
                    "UPDATE chatbot_outbox SET status = 'processing' WHERE id = ?",
                    params_from_iter(vec![Value::Integer(msg.id)]),
                )
                .await
            {
                let _ = conn
                    .execute("ROLLBACK", params_from_iter(Vec::<Value>::new()))
                    .await;
                return Err(e.into());
            }
        }

        conn.execute("COMMIT", params_from_iter(Vec::<Value>::new()))
            .await?;
        Ok(messages)
    }

    pub async fn mark_sent(&self, id: i64) -> anyhow::Result<()> {
        let conn = self.db.connect().await?;
        let sql = "UPDATE chatbot_outbox SET status = 'sent', sent_at = unixepoch() WHERE id = ?";
        conn.execute(sql, params_from_iter(vec![Value::Integer(id)]))
            .await?;
        Ok(())
    }

    pub async fn mark_failed(
        &self,
        id: i64,
        error_message: &str,
        next_retry_at: i64,
    ) -> anyhow::Result<()> {
        let conn = self.db.connect().await?;
        let mut stmt = conn
            .prepare("SELECT attempts FROM chatbot_outbox WHERE id = ?")
            .await?;
        let mut rows = stmt
            .query(params_from_iter(vec![Value::Integer(id)]))
            .await?;
        let attempts: i64 = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            0
        };
        let new_attempts = attempts + 1;
        let status = if new_attempts >= 5 {
            "failed"
        } else {
            "pending"
        };

        let sql = "UPDATE chatbot_outbox SET status = ?, attempts = ?, error_message = ?, next_retry_at = ? WHERE id = ?";
        let params = vec![
            Value::Text(status.to_string()),
            Value::Integer(new_attempts),
            Value::Text(error_message.to_string()),
            Value::Integer(next_retry_at),
            Value::Integer(id),
        ];
        conn.execute(sql, params_from_iter(params)).await?;
        Ok(())
    }
}
