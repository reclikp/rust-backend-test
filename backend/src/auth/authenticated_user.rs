use rocket::http::Status;
use crate::middleware::response_models::ErrorResponse;
use entity::user::Model as User;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
    pub roles: Vec<String>,
}

impl AuthenticatedUser {
    pub fn has_role(&self, role: &str) -> Result<(), ErrorResponse> {
        if self.roles.contains(&role.to_string()) {
            Ok({})
        } else {
            Err(ErrorResponse::new(Status::Unauthorized, "User doesn't have sufficient permissions"))
        }
    }
}
