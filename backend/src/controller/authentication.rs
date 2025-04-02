use rocket::Route;
use crate::middleware::network_response::NetworkResponse;

use crate::middleware::jwt::JWT;

pub fn get_routes() -> Vec<Route> {
    routes![
        jebanko,
    ]
}

#[get("/jebanko")]
pub fn jebanko(
    // key: Result<JWT, NetworkResponse>,
) -> Result<String, NetworkResponse> {
    Ok("ok".to_string())
}
