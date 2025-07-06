use std::env;

pub fn get_app_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
