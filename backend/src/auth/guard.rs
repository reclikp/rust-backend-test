use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use crate::auth::authenticated_user::AuthenticatedUser;
use crate::auth::jwt::{decode_jwt, JWT};
use crate::config::app_state::AppState;
use crate::middleware::response_models::NetworkResponse;

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = NetworkResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header = request.headers().get_one("Authorization");
        let repository = &request
            .rocket()
            .state::<AppState>()
            .unwrap()
            .user_repository;

        match authorization_header {
            None => Outcome::Error((
                Status::Unauthorized,
                NetworkResponse::Unauthorized("Missing Authorization header".to_string()),
            )),
            Some(key) => match decode_jwt(key) {
                Ok(claims) => match repository.find_by_id(claims.subject_id).await {
                    Some(user) => {
                        let roles: Vec<String> =
                            serde_json::from_value(user.roles.clone()).unwrap_or_default();

                        Outcome::Success(AuthenticatedUser { user, roles })
                    }
                    None => Outcome::Error((
                        Status::Unauthorized,
                        NetworkResponse::Unauthorized("User not found".to_string()),
                    )),
                },
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
