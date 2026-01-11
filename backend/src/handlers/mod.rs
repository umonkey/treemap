//! This module loads all action handlers.
//! Handlers are where the actual logic of the API is implemented.
//! Actions just receive input and provide output to the user.
//! This is mor or less the ADR pattern.

mod osm_push_changes_handler;
mod osm_push_handler;
pub use osm_push_changes_handler::*;
pub use osm_push_handler::*;
