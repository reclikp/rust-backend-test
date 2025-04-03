use sea_orm::DatabaseConnection;
use crate::config::database;
use crate::repository::user_repository::UserRepository;
use crate::service::authentication::AuthenticationService;

pub struct AppState {
    pub database_connection: DatabaseConnection,

    pub user_repository: UserRepository,

    pub authentication_service: AuthenticationService,
}

impl AppState {
    pub async fn new() -> Self {
        let database_connection = database::connect().await;

        let user_repository = UserRepository::new(database_connection.clone());

        let authentication_service = AuthenticationService::new(user_repository.clone());

        AppState {
            database_connection,
            user_repository,
            authentication_service
        }
    }
}
