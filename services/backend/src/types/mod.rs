mod errors;
pub type Result<T> = std::result::Result<T, self::Error>;
pub use self::errors::Error;
