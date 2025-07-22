use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}
