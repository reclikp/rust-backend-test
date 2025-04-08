use crate::config::app_state::AppState;
use crate::middleware::response_models::{NetworkResponse, ResponseBody};
use entity::user;
use rocket::serde::json::Json;
use rocket::{Route, State};

use crate::middleware::jwt::JWT;
use crate::middleware::request_models::AuthenticationRequest;
use crate::service;
use crate::service::authentication::AuthenticationService;

pub fn get_routes() -> Vec<Route> {
    routes![jebanko, authenticate,]
}

#[get("/jebanko")]
fn jebanko(_jwt: JWT) -> Result<String, NetworkResponse> {
    Ok("ok".to_string())
}

#[post("/authenticate", format = "json", data = "<request>")]
async fn authenticate(
    request: Json<AuthenticationRequest>,
    app_state: &State<AppState>,
) -> Result<NetworkResponse, NetworkResponse> {
    dbg!(&request);
    let result = app_state.authentication_service.authenticate(request.into_inner()).await;

    let response = Response {
        body: ResponseBody::AuthenticationToken(result),
    }

    Ok(NetworkResponse::Created("Ok".to_string()))
}
