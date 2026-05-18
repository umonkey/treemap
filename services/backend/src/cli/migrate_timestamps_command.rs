use crate::domain::tree::TreeRepository;
use crate::infra::database::Value;
use crate::services::{AppState, Context, ContextExt};
use log::info;

pub async fn migrate_timestamps_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let db = state.database();
    let trees_repo = state
        .build::<TreeRepository>()
        .expect("Error building TreeRepository.");

    info!("Starting timestamp migration...");

    let trees = trees_repo.all().await.expect("Error fetching all trees.");
    info!("Found {} trees to process.", trees.len());

    for tree in trees {
        let mut updates: Vec<(&str, u64)> = Vec::new();

        // 1. Metrics from trees_props
        for prop in &["height", "diameter", "circumference"] {
            let sql = "SELECT MAX(added_at) as last_update FROM trees_props WHERE tree_id = ? AND name = ?";
            let params = &[Value::from(tree.id as i64), Value::from(prop.to_string())];
            let rows = db
                .fetch_sql(sql, params)
                .await
                .expect("Error fetching props.");
            if let Some(row) = rows.first() {
                if let Ok(Some(ts)) = row.get_u64("last_update") {
                    let field_name = match *prop {
                        "height" => "height_updated_at",
                        "diameter" => "diameter_updated_at",
                        "circumference" => "circumference_updated_at",
                        _ => unreachable!(),
                    };
                    updates.push((field_name, ts));
                }
            }
        }

        // 2. Images from trees_images
        let sql = "SELECT MAX(added_at) as last_update FROM trees_images WHERE tree_id = ?";
        let params = &[Value::from(tree.id as i64)];
        let rows = db
            .fetch_sql(sql, params)
            .await
            .expect("Error fetching images.");
        if let Some(row) = rows.first() {
            if let Ok(Some(ts)) = row.get_u64("last_update") {
                updates.push(("images_updated_at", ts));
            }
        }

        // 3. Observations from observations
        let sql = "SELECT MAX(created_at) as last_update FROM observations WHERE tree_id = ?";
        let params = &[Value::from(tree.id as i64)];
        let rows = db
            .fetch_sql(sql, params)
            .await
            .expect("Error fetching observations.");
        if let Some(row) = rows.first() {
            if let Ok(Some(ts)) = row.get_u64("last_update") {
                updates.push(("observations_updated_at", ts));
            }
        }

        if !updates.is_empty() {
            let mut sql = "UPDATE trees SET ".to_string();
            let mut sets = Vec::new();
            let mut params = Vec::new();

            for (field, ts) in updates {
                sets.push(format!("{} = ?", field));
                params.push(Value::from(ts as i64));
            }

            sql.push_str(&sets.join(", "));
            sql.push_str(" WHERE id = ?");
            params.push(Value::from(tree.id as i64));

            db.execute_sql(&sql, &params)
                .await
                .expect("Error updating tree.");
        }
    }

    info!("Timestamp migration completed.");
}
