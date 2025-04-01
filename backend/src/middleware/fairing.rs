use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use rocket::data::Outcome;
use rocket::form::validate::Contains;
use rocket::http::Status;
use crate::middleware::jwt::{decode_jwt, JWT};
use crate::middleware::network_response::NetworkResponse;

pub struct AuthenticationFairing;

#[rocket::async_trait]
impl Fairing for AuthenticationFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT Authenticator",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) -> Outcome<Self, String> {
        dbg!(&_req);

        // fn is_valid(key: &str) -> Result<Claims, Error> {
        //     Ok(decode_jwt(String::from(key))?)
        // }

        let authorization_header = _req.headers().get_one("Authorization");

        match authorization_header {
            None => {
                Outcome::Error((Status::Unauthorized, "Missing Authorization header".to_string()))
            },
            Some(key) => match decode_jwt(key.to_string()) {
                Ok(claims) => Outcome::Success((Status::Accepted, format!("JWT {}", claims))),
                Err(error) => match error {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        Outcome::Error((Status::Unauthorized, "The token has expired".to_string()))
                    }
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        Outcome::Error((Status::Unauthorized, "The token is invalid".to_string()))
                    }
                    _ => {
                        Outcome::Error((Status::Unauthorized, "The token is not working".to_string()))
                    }
                }
            }
        }
    }
}
