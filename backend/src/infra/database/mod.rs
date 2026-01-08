//! This module contains code that talks to different databases,
//! and the code needed to abstract from them.

mod attributes;
mod base;
mod interface;
mod sqlite_database;
mod value;

pub use attributes::Attributes;
pub use interface::Database;
pub use value::Value;
