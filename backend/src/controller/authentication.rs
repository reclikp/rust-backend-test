use rocket::{Route, State};
use entity::user;
use crate::config::app_state::AppState;
use crate::middleware::network_response::NetworkResponse;

use crate::middleware::jwt::JWT;
use crate::service;
use crate::service::authentication::AuthenticationService;

pub fn get_routes() -> Vec<Route> {
    routes![
        jebanko,
        authenticate,
    ]
}

#[get("/jebanko")]
fn jebanko(_jwt: JWT) -> Result<String, NetworkResponse> {
    Ok("ok".to_string())
}

#[post("/authenticate")]
async fn authenticate(
    app_state: &State<AppState>,
    // service: &State<AuthenticationService>,
) -> Result<NetworkResponse, NetworkResponse> {
    // let result = service.authenticate().await;
    //
    // dbg!(result);

    // let dupa = service::authentication::authenticate().await;

    Ok(NetworkResponse::Created("Ok".to_string()))
}
