use crate::repository::user_repository::UserRepository;

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
}

impl AuthenticationService {
    pub fn new(user_repository: UserRepository) -> Self {
        AuthenticationService { user_repository }
    }

    pub async fn authenticate(&self) -> String {
        "OK".to_string()
    }
}
