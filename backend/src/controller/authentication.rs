use crate::middleware::network_response::NetworkResponse;

use crate::middleware::jwt::JWT;

#[get("/jebanko")]
pub fn jebanko(
    // key: Result<JWT, NetworkResponse>,
) -> Result<String, NetworkResponse> {



    Result::Ok("ok".to_string())
}
