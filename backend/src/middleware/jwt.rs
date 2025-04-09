use crate::middleware::response_models::NetworkResponse;
use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::Request;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header = request.headers().get_one("Authorization");

        match authorization_header {
            None => Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized("Missing Authorization header".to_string()),
            )),
            Some(key) => match decode_jwt(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(error) => match error {
                    ErrorKind::ExpiredSignature => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized("The token has expired".to_string()),
                    )),
                    ErrorKind::InvalidToken => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized("The token is invalid".to_string()),
                    )),
                    _ => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized(
                            "The authorization is not working".to_string(),
                        ),
                    )),
                },
            },
        }
    }
}
