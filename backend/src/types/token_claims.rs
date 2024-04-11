/**
 * This is the JWT token payload.
 */

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: u64,
}
