use api::response_models::{ErrorResponse, StatusCode};
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
            Err(ErrorResponse::new(StatusCode::FORBIDDEN, "User doesn't have sufficient permissions"))
        }
    }
}
