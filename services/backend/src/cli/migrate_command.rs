//! Apply forward-only SQL migrations recorded in `schema_migrations`.

use crate::infra::database::{Database, Value};
use crate::services::{AppState, Context};
use crate::types::Result;
use crate::utils::get_timestamp;
use log::info;

struct Migration {
    id: &'static str,
    sql: &'static str,
}

const MIGRATIONS: &[Migration] = &[
    Migration {
        id: "001-rbac",
        sql: include_str!("../../dev/migrations/001-rbac.sql"),
    },
    Migration {
        id: "002-instances",
        sql: include_str!("../../dev/migrations/002-instances.sql"),
    },
    Migration {
        id: "003-panoramas",
        sql: include_str!("../../dev/migrations/003-panoramas.sql"),
    },
    Migration {
        id: "004-chatbot-alerts",
        sql: include_str!("../../dev/migrations/004-chatbot-alerts.sql"),
    },
    Migration {
        id: "005-chatbot-alerts-reported-at",
        sql: include_str!("../../dev/migrations/005-chatbot-alerts-reported-at.sql"),
    },
    Migration {
        id: "006-chatbot-alerts-draft",
        sql: include_str!("../../dev/migrations/006-chatbot-alerts-draft.sql"),
    },
    Migration {
        id: "007-chatbot-outbox-attachments",
        sql: include_str!("../../dev/migrations/007-chatbot-outbox-attachments.sql"),
    },
    Migration {
        id: "008-tree-state-alive",
        sql: include_str!("../../dev/migrations/008-tree-state-alive.sql"),
    },
    Migration {
        id: "009-chatbot-alerts-ping-at",
        sql: include_str!("../../dev/migrations/009-chatbot-alerts-ping-at.sql"),
    },
    Migration {
        id: "010-drop-panorama-transcode",
        sql: include_str!("../../dev/migrations/010-drop-panorama-transcode.sql"),
    },
    Migration {
        id: "011-drop-panorama-gpx-offset",
        sql: include_str!("../../dev/migrations/011-drop-panorama-gpx-offset.sql"),
    },
    Migration {
        id: "012-drop-panorama-web-video-path",
        sql: include_str!("../../dev/migrations/012-drop-panorama-web-video-path.sql"),
    },
    Migration {
        id: "013-panorama-file-size",
        sql: include_str!("../../dev/migrations/013-panorama-file-size.sql"),
    },
    Migration {
        id: "014-panorama-processing-time",
        sql: include_str!("../../dev/migrations/014-panorama-processing-time.sql"),
    },
    Migration {
        id: "015-water-source",
        sql: include_str!("../../dev/migrations/015-water-source.sql"),
    },
    Migration {
        id: "016-panorama-distance",
        sql: include_str!("../../dev/migrations/016-panorama-distance.sql"),
    },
    Migration {
        id: "017-chatbot-outbox-topic-id",
        sql: include_str!("../../dev/migrations/017-chatbot-outbox-topic-id.sql"),
    },
    Migration {
        id: "018-panorama-hints-created-at",
        sql: include_str!("../../dev/migrations/018-panorama-hints-created-at.sql"),
    },
    Migration {
        id: "019-drop-chatbot-outbox-alert-id",
        sql: include_str!("../../dev/migrations/019-drop-chatbot-outbox-alert-id.sql"),
    },
    Migration {
        id: "020-instance-bbox",
        sql: include_str!("../../dev/migrations/020-instance-bbox.sql"),
    },
    Migration {
        id: "021-species-manage",
        sql: include_str!("../../dev/migrations/021-species-manage.sql"),
    },
    Migration {
        id: "022-instance-manage",
        sql: include_str!("../../dev/migrations/022-instance-manage.sql"),
    },
    Migration {
        id: "023-instance-osm",
        sql: include_str!("../../dev/migrations/023-instance-osm.sql"),
    },
    Migration {
        id: "024-alert-web-submission",
        sql: include_str!("../../dev/migrations/024-alert-web-submission.sql"),
    },
    Migration {
        id: "025-alert-create",
        sql: include_str!("../../dev/migrations/025-alert-create.sql"),
    },
    Migration {
        id: "026-alert-manage",
        sql: include_str!("../../dev/migrations/026-alert-manage.sql"),
    },
    Migration {
        id: "027-instance-feature-flags",
        sql: include_str!("../../dev/migrations/027-instance-feature-flags.sql"),
    },
    Migration {
        id: "028-file-storage",
        sql: include_str!("../../dev/migrations/028-file-storage.sql"),
    },
];

pub async fn migrate_command() {
    let baseline = std::env::args().any(|arg| arg == "--baseline");

    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let db = state.database();

    ensure_table(&db)
        .await
        .expect("Error creating schema_migrations.");

    if baseline {
        baseline_existing(&db)
            .await
            .expect("Error recording baseline migrations.");
    }

    apply_pending(&db)
        .await
        .expect("Error applying migrations.");
}

async fn ensure_table(db: &Database) -> Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS `schema_migrations` (`id` TEXT NOT NULL, `applied_at` INT NOT NULL, PRIMARY KEY(`id`))";
    db.execute_sql(sql, &[]).await?;
    Ok(())
}

async fn applied_ids(db: &Database) -> Result<Vec<String>> {
    let rows = db
        .fetch_sql("SELECT `id` FROM `schema_migrations`", &[])
        .await?;

    rows.iter()
        .map(|row| row.require_string("id"))
        .collect::<Result<Vec<String>>>()
}

async fn baseline_existing(db: &Database) -> Result<()> {
    let now = get_timestamp();

    for migration in MIGRATIONS {
        if migration.id == "028-file-storage" {
            continue;
        }

        let sql = "INSERT OR IGNORE INTO `schema_migrations` (`id`, `applied_at`) VALUES (?, ?)";
        db.execute_sql(sql, &[Value::from(migration.id), Value::from(now as i64)])
            .await?;
    }

    info!("Recorded migrations 001-027 as applied.");

    Ok(())
}

async fn apply_pending(db: &Database) -> Result<()> {
    let applied = applied_ids(db).await?;

    for migration in MIGRATIONS {
        if applied.iter().any(|id| id == migration.id) {
            continue;
        }

        info!("Applying migration {}.", migration.id);

        apply_migration(db, migration).await?;

        let sql = "INSERT INTO `schema_migrations` (`id`, `applied_at`) VALUES (?, ?)";
        db.execute_sql(
            sql,
            &[
                Value::from(migration.id),
                Value::from(get_timestamp() as i64),
            ],
        )
        .await?;
    }

    Ok(())
}

async fn apply_migration(db: &Database, migration: &Migration) -> Result<()> {
    if migration.id != "028-file-storage" {
        db.execute_batch(migration.sql).await?;
        return Ok(());
    }

    // 028 is also part of the full base schema, so make it safe to re-run.
    for statement in split_statements(migration.sql) {
        let upper = statement.to_uppercase();

        if upper.starts_with("ALTER TABLE")
            && upper.contains("ADD COLUMN")
            && column_exists(db, "trees", "thumbnail_file_id").await?
        {
            continue;
        }

        db.execute_sql(&statement, &[]).await?;
    }

    Ok(())
}

fn split_statements(sql: &str) -> Vec<String> {
    sql.split(';')
        .map(|statement| statement.trim().to_string())
        .filter(|statement| !statement.is_empty())
        .collect()
}

async fn column_exists(db: &Database, table: &str, column: &str) -> Result<bool> {
    let sql = format!("PRAGMA table_info(`{table}`)");
    let rows = db.fetch_sql(&sql, &[]).await?;

    for row in rows {
        if let Ok(name) = row.require_string("name") {
            if name == column {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
