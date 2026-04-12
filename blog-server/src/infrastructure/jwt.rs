use crate::domain::{error::ServerError};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const JWT_TTL_MINUTES: i64 = 1440;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub user_id: i64, 
    pub username: String, 
    pub exp: i64
}

impl Default for Claims {
    fn default() -> Self {
        Self::new()
    }
}

impl Claims {
    pub fn new() -> Self {
        Self {
            user_id: 0, 
            username: String::new(),
            exp: 0
        }
    }
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
        }
    }

    pub fn generate_token(&self, user_id: i64, username: &str) -> Result<String, ServerError> {
        match encode(
            &Header::default(),
            &Claims {
                user_id: user_id,
                username: username.to_owned(),
                exp: (Utc::now() + Duration::minutes(JWT_TTL_MINUTES)).timestamp(),
            },
            &self.encoding_key
        ) {
            Ok(token) => return Ok(token),
            Err(e) => return Err(ServerError::CodingEncodingError(e)),
        }
    }

    pub fn verify_token(self, token: &str) -> Result<Claims, ServerError> {

        match decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        ) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(ServerError::InvalidCredentialsError)
        }
    }
}