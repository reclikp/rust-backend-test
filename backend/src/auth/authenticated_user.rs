use crate::middleware::response_models::NetworkResponse;
use entity::user::Model as User;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
    pub roles: Vec<String>,
}

impl AuthenticatedUser {
    pub fn has_role(&self, role: &str) -> Result<(), NetworkResponse> {
        if self.roles.contains(&role.to_string()) {
            Ok(())
        } else {
            Err(NetworkResponse::Unauthorized("User doesn't have sufficient permissions".to_string()))
        }
    }
}
