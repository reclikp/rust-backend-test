use jsonwebtoken::errors::ErrorKind;
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::serde_json;
use crate::auth::authenticated_user::AuthenticatedUser;
use crate::auth::jwt::{decode_jwt, JWT};
use crate::config::app_state::AppState;
use crate::middleware::response_models::{ErrorResponse, StatusCode};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = ErrorResponse;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let authorization_header = request.headers().get_one("Authorization");

        match authorization_header {
            None => Outcome::Error((
                Status::Unauthorized,
                ErrorResponse::new(StatusCode::UNAUTHORIZED, "Missing Authorization header"),
            )),
            Some(key) => match decode_jwt(key) {
                Ok(claims) => Outcome::Success(JWT { claims }),
                Err(error) => jwt_error_kind_outcome_error(error),
            },
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ErrorResponse;

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
                ErrorResponse::new(StatusCode::UNAUTHORIZED, "Missing Authorization header"),
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
                        ErrorResponse::new(StatusCode::UNAUTHORIZED, "User not found"),
                    )),
                },
                Err(error) => jwt_error_kind_outcome_error(error),
            },
        }
    }
}

fn jwt_error_kind_outcome_error<'r, T>(error: ErrorKind) -> Outcome<T, ErrorResponse> {
    match error {
        ErrorKind::ExpiredSignature => Outcome::Error((
            Status::Unauthorized,
            ErrorResponse::new(StatusCode::UNAUTHORIZED, "The token has expired"),
        )),
        ErrorKind::InvalidToken => Outcome::Error((
            Status::Unauthorized,
            ErrorResponse::new(StatusCode::UNAUTHORIZED, "The token is invalid"),
        )),
        _ => Outcome::Error((
            Status::Unauthorized,
            ErrorResponse::new(StatusCode::UNAUTHORIZED, "The authorization is not working"),
        )),
    }
}
