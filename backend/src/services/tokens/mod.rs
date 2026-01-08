use crate::config::Secrets;
use crate::services::{Locatable, Locator};
use crate::types::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{debug, error};
use std::sync::Arc;

#[derive(Clone)]
pub struct TokenService {
    sym_enc: EncodingKey,
    sym_dec: DecodingKey,
}

impl TokenService {
    pub fn new(secrets: Arc<Secrets>) -> Self {
        let secret = secrets.jwt_secret.clone().unwrap_or("secret".to_string());

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
        Ok(Self::new(locator.get::<Secrets>()?))
    }
}
