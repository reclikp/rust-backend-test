use std::env;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::{Error, ErrorKind};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub subject_id: i32,
    expiration: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

pub fn create_jwt(subject_id: i32) -> Result<String, Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60)).unwrap().timestamp() as usize;

    let claims = Claims {
        subject_id,
        expiration,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token = token.trim_start_matches("Bearer").trim();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned())
    }
}
