use crate::middleware::jwt::create_jwt;
use crate::middleware::request_models::AuthenticationRequest;
use crate::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
}

impl AuthenticationService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn authenticate(&self, request: AuthenticationRequest) -> Option<String> {
        if let Some(user) = self
            .user_repository
            .find_by_username(&request.username)
            .await
        {
            create_jwt(user.id).ok()
        } else {
            None
        }
    }
}
