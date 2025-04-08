use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::middleware::jwt::{decode_jwt, Claims, JWT};
use crate::middleware::response_models::NetworkResponse;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // fn is_valid(key: &str) -> Result<Claims, ErrorKind> {
        //     decode_jwt(String::from(key))
        // }

        dbg!(request);

        let authorization_header = request.headers().get_one("Authorization");

        match authorization_header {
            None => {
                Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized("Missing Authorization header".to_string())))
            },
            Some(key) => match decode_jwt(key.to_string()) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(error) => match error {
                    ErrorKind::ExpiredSignature => {
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized("The token has expired".to_string())))
                    }
                    ErrorKind::InvalidToken => {
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized("The token is invalid".to_string())))
                    }
                    _ => {
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized("The authorization is not working".to_string())))
                    }
                }
            }
        }
    }
}
