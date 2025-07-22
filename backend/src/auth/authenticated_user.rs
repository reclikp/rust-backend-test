use entity::user::Model as User;
use crate::middleware::response_models::{ErrorResponse, StatusCode};

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
    pub roles: Vec<String>,
}

impl AuthenticatedUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(&role.to_string())
    }
}
