use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::request::{FromRequest};
use rocket::serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub subject_id: i32,
    #[serde(rename = "exp")]
    expiration: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

pub fn create_jwt(subject_id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration_seconds = env::var("JWT_EXPIRATION_SECONDS")
        .expect("JWT_EXPIRATION_SECONDS must be set")
        .parse::<i64>()
        .unwrap();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expiration_seconds))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        subject_id,
        expiration,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}
