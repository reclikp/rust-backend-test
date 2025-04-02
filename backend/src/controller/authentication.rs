use rocket::{Route, State};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use entity::user;
use crate::service::authentication as AuthenticationService;
use crate::middleware::network_response::NetworkResponse;

use crate::middleware::jwt::JWT;
use crate::service;

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
    db: &State<DatabaseConnection>
) -> Result<NetworkResponse, NetworkResponse> {
    let db = db as &DatabaseConnection;

    let dupa = service::authentication::authenticate().await;

    Ok(NetworkResponse::Created("Ok".to_string()))
}
