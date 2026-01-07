//! This module contains the code that performs health checks.
//!
//! To tell that we are safe, it needs to test the database and the
//! file storage by requesting some data.  Only the components which
//! are actually required for the API to run and work, excluding all
//! background tasks like OSM sync.  Overpass being down should not
//! prevent the API and our web UI from working.

mod actions;
mod domain;

pub use actions::*;
pub use domain::*;
