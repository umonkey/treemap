mod dates;
mod env;
mod id;
mod strings;
pub use dates::*;
pub use env::*;
pub use id::*;
pub use strings::*;

pub fn fix_circumference(value: Option<f64>) -> Option<f64> {
    let mut value: f64 = match value {
        Some(v) => v,
        None => return None,
    };

    if value.fract() == 0.0 && value > 3.0 {
        value /= 100.0;
    }

    Some(value)
}
