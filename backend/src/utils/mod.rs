use std::env;

pub fn getenv_usize(name: &str, default: usize) -> usize {
    match env::var(name) {
        Ok(v) => v.parse::<usize>().unwrap_or(default),
        Err(_) => default,
    }
}

pub fn getenv_u16(name: &str, default: u16) -> u16 {
    match env::var(name) {
        Ok(v) => v.parse::<u16>().unwrap_or(default),
        Err(_) => default,
    }
}

pub fn getenv_string(name: &str, default: &str) -> String {
    match env::var(name) {
        Ok(v) => v,
        Err(_) => default.to_string(),
    }
}
