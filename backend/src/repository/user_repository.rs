use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use entity::user;

#[derive(Clone)]
pub struct UserRepository {
    connection: DatabaseConnection,
}

impl UserRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        UserRepository { connection }
    }

    pub async fn find_by_username(&self, username: &str) -> Option<user::Model> {
        let user = user::Entity::find()
            .filter(user::Column::Username.eq("pjomter"))
            .one(&self.connection)
            .await;

        dbg!(&user);

        user.unwrap()
    }
}
