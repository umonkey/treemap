//! This module loads all action handlers.
//! Handlers are where the actual logic of the API is implemented.
//! Actions just receive input and provide output to the user.
//! This is mor or less the ADR pattern.

mod add_photo_handler;
mod osm_push_changes_handler;
mod osm_push_handler;
mod resize_image_handler;
mod update_settings_handler;
mod update_tree_address_handler;
mod update_tree_addresses_handler;
mod upload_local_files;
pub use add_photo_handler::*;
pub use osm_push_changes_handler::*;
pub use osm_push_handler::*;
pub use resize_image_handler::*;
pub use update_settings_handler::*;
pub use update_tree_address_handler::*;
pub use update_tree_addresses_handler::*;
pub use upload_local_files::*;
