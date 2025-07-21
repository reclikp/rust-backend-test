use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::auth::jwt::{decode_jwt, JWT};
use crate::middleware::response_models::ErrorResponse;
// use crate::middleware::response_models::NetworkResponse;
//
// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for JWT {
//     type Error = NetworkResponse;
//
//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         let authorization_header = request.headers().get_one("Authorization");
//
//         match authorization_header {
//             None => Outcome::Error((
//                 Status::Unauthorized,
//                 NetworkResponse::Unauthorized("Missing Authorization header".to_string()),
//             )),
//             Some(key) => match decode_jwt(key) {
//                 Ok(claims) => Outcome::Success(JWT { claims }),
//                 Err(error) => match error {
//                     ErrorKind::ExpiredSignature => Outcome::Error((
//                         Status::Unauthorized,
//                         NetworkResponse::Unauthorized("The token has expired".to_string()),
//                     )),
//                     ErrorKind::InvalidToken => Outcome::Error((
//                         Status::Unauthorized,
//                         NetworkResponse::Unauthorized("The token is invalid".to_string()),
//                     )),
//                     _ => Outcome::Error((
//                         Status::Unauthorized,
//                         NetworkResponse::Unauthorized(
//                             "The authorization is not working".to_string(),
//                         ),
//                     )),
//                 },
//             },
//         }
//     }
// }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header = request.headers().get_one("Authorization");

        match authorization_header {
            None => Outcome::Error((
                Status::Unauthorized,
                ErrorResponse::new(Status::Unauthorized, "Missing Authorization header"),
            )),
            Some(key) => match decode_jwt(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(error) => match error {
                    ErrorKind::ExpiredSignature => Outcome::Error((
                        Status::Unauthorized,
                        ErrorResponse::new(Status::Unauthorized, "The token has expired"),
                    )),
                    ErrorKind::InvalidToken => Outcome::Error((
                        Status::Unauthorized,
                        ErrorResponse::new(Status::Unauthorized, "The token is invalid"),
                    )),
                    _ => Outcome::Error((
                        Status::Unauthorized,
                        ErrorResponse::new(Status::Unauthorized, "The authorization is not working"),
                    )),
                },
            },
        }
    }
}
