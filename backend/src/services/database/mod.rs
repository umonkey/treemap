/**
 * This module contains different database implementations.
 *
 * Currently there's only SQLite, but we plan to add more soon, at least DynamoDB.
 */

mod sqlite;
mod r#trait;

pub use self::r#trait::*;
pub use self::sqlite::*;

use std::sync::Arc;
use crate::Result;

pub async fn get_database() -> Result<Arc<dyn Database>> {
    Ok(Arc::new(SqliteDatabase::new().await?))
}
