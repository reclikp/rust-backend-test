use crate::config::app_state::AppState;
use rocket::serde::json::Json;
use rocket::{Route, State};
use rocket::http::Status;
use serde::Serialize;
use api::request_models::AuthenticationRequest;
use api::response_models::{ErrorResponse, Response, StatusCode};
use crate::auth::jwt::JWT;
use crate::service;
use crate::service::authentication::AuthenticationService;

pub fn get_routes() -> Vec<Route> {
    routes![
        authenticate,
    ]
}

#[derive(Serialize)]
struct AuthenticationResponse {
    token: String
}

#[post("/authenticate", format = "json", data = "<request>")]
async fn authenticate(
    request: Json<AuthenticationRequest>,
    app_state: &State<AppState>,
) -> Result<Response<AuthenticationResponse>, ErrorResponse> {
    let result = app_state.authentication_service.authenticate(request.into_inner()).await;

    match result {
        Some(token) => {
            let response = Response::new(AuthenticationResponse { token });

            Ok(response)
        }
        None => Err(ErrorResponse::new(StatusCode::UNAUTHORIZED, "Invalid credentials"))
    }
}
