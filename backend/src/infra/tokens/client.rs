use super::types::TokenClaims;
use crate::config::Secrets;
use crate::services::{Locatable, Locator};
use crate::types::{Error, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error};

#[derive(Clone)]
pub struct TokenService {
    sym_enc: EncodingKey,
    sym_dec: DecodingKey,
}

impl TokenService {
    pub fn new(secret: String) -> Self {
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
                error!("Error decoding token: {e}");
                debug!("Token payload: {encoded}");
                Err(e.into())
            }
        }
    }
}

impl Locatable for TokenService {
    fn create(locator: &Locator) -> Result<Self> {
        let secrets = locator.get::<Secrets>()?;
        let jwt_secret = secrets.jwt_secret.clone().ok_or(Error::Config(
            "JWT_SECRET not set, cannot use tokens".to_string(),
        ))?;
        Ok(Self::new(jwt_secret))
    }
}
