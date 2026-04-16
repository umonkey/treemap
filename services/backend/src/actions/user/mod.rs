//! This module groups web actions related to user management.

mod actions;
mod schemas;

pub use actions::user_router;
pub use schemas::UserList;
pub use schemas::UserRead;
