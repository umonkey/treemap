/**
 * This module contains different database implementations.
 *
 * Currently there's only SQLite, but we plan to add more soon, at least DynamoDB.
 */

mod sqlite;

pub use self::sqlite::*;
