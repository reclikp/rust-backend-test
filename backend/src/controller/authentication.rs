use crate::config::app_state::AppState;
use crate::middleware::response_models::{NetworkResponse, Response, ResponseBody};
use entity::user;
use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::middleware::jwt::JWT;
use crate::middleware::request_models::AuthenticationRequest;
use crate::service;
use crate::service::authentication::AuthenticationService;

pub fn get_routes() -> Vec<Route> {
    routes![
        authenticate,
    ]
}

#[post("/authenticate", format = "json", data = "<request>")]
async fn authenticate(
    request: Json<AuthenticationRequest>,
    app_state: &State<AppState>,
) -> Result<NetworkResponse, NetworkResponse> {
    dbg!(&request);
    let result = app_state.authentication_service.authenticate(request.into_inner()).await;

    match result {
        Some(token) => {
            let response = Response {
                body: ResponseBody::AuthenticationToken(token),
            };

            let json = serde_json::to_string(&response).map_err(|_| NetworkResponse::InternalError(None))?;

            Ok(NetworkResponse::Created(json))
        }
        None => Err(NetworkResponse::Unauthorized("Invalid credentials".to_string()))
    }

    // let response = Response {
    //     body: ResponseBody::AuthenticationToken(result),
    // }
    //
    // Ok(NetworkResponse::Created("Ok".to_string()))
}
