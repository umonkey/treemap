//! This service enriches alerts with platform reporter info.

use super::schemas::{AlertFull, AlertList};
use crate::domain::alert::Alert;
use crate::domain::user::UserRepository;
use crate::services::{Context, Injectable};
use crate::types::Result;
use std::sync::Arc;

pub struct AlertLoader {
    users: Arc<UserRepository>,
}

impl AlertLoader {
    pub async fn load(&self, alerts: &[Alert]) -> Result<AlertList> {
        let user_ids = reporter_ids(alerts);
        let users = self.users.get_multiple(&user_ids).await?;

        Ok(AlertList::from_records(alerts, &users))
    }

    pub async fn load_one(&self, alert: &Alert) -> Result<AlertFull> {
        let users = if alert.chat_id.is_none() {
            self.users.get_multiple(&[alert.created_by]).await?
        } else {
            Vec::new()
        };

        Ok(AlertFull::from_record(alert, &users))
    }
}

impl Injectable for AlertLoader {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            users: Arc::new(ctx.build::<UserRepository>()?),
        })
    }
}

/// Platform reporter ids are meaningful only for web alerts. Telegram rows
/// store a Telegram user id in `created_by`, which must never be joined to
/// the platform `users` table.
fn reporter_ids(alerts: &[Alert]) -> Vec<u64> {
    alerts
        .iter()
        .filter(|alert| alert.chat_id.is_none())
        .map(|alert| alert.created_by)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn web_alert(created_by: u64) -> Alert {
        Alert {
            created_by,
            ..Default::default()
        }
    }

    fn telegram_alert(created_by: u64) -> Alert {
        Alert {
            created_by,
            chat_id: Some(42),
            ..Default::default()
        }
    }

    #[test]
    fn reporter_ids_includes_only_web_alert_creators() {
        let alerts = vec![web_alert(42), telegram_alert(42), web_alert(7)];

        assert_eq!(reporter_ids(&alerts), vec![42, 7]);
    }

    #[test]
    fn reporter_ids_is_empty_without_web_alerts() {
        let alerts = vec![telegram_alert(42), telegram_alert(7)];

        assert!(reporter_ids(&alerts).is_empty());
    }
}
