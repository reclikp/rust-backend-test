use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::middleware::jwt::{decode_jwt, Claims, JWT};
use crate::middleware::network_response::NetworkResponse;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // fn is_valid(key: &str) -> Result<Claims, ErrorKind> {
        //     decode_jwt(String::from(key))
        // }

        dbg!(request);

        // todo!();

        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized("dupa kurwa chuj".to_string())))
    }
}
