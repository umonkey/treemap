mod dates;
mod env;
mod id;
mod osm;
mod requests;
mod strings;
pub use dates::*;
pub use env::*;
pub use id::*;
pub use osm::*;
pub use requests::*;
pub use strings::*;

pub fn fix_circumference(value: Option<f64>) -> Option<f64> {
    let mut value: f64 = value?;

    if value.fract() == 0.0 && value > 3.0 {
        value /= 100.0;
    }

    Some(value)
}
