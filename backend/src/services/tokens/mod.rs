use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error};

use crate::types::{Result, TokenClaims};
use crate::utils::get_jwt_secret;

#[derive(Clone)]
pub struct TokenService {
    sym_enc: EncodingKey,
    #[allow(unused)]
    sym_dec: DecodingKey,
}

impl TokenService {
    pub fn new() -> Self {
        let secret = get_jwt_secret();

        Self {
            sym_enc: EncodingKey::from_secret(secret.as_ref()),
            sym_dec: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn encode(&self, claims: &TokenClaims) -> Result<String> {
        encode(&Header::default(), claims, &self.sym_enc).map_err(|e| e.into())
    }

    #[allow(unused)]
    pub fn decode(&self, encoded: &str) -> Result<TokenClaims> {
        match decode::<TokenClaims>(encoded, &self.sym_dec, &Validation::default()) {
            Ok(token) => Ok(token.claims),

            Err(e) => {
                error!("Error decoding token: {}", e);
                debug!("Token payload: {}", encoded);
                Err(e.into())
            }
        }
    }
}
