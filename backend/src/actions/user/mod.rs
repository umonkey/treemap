//! This module groups web actions related to user management.

mod actions;
mod schemas;

pub use actions::get_top_users;
pub use actions::user_router;
pub use schemas::UserRead;
