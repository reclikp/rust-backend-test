use rocket::serde::{Deserialize, Serialize};
use sea_orm::JsonValue;
use sea_orm::prelude::*;

#[derive(DeriveEntityModel, Serialize, Deserialize, Debug, Clone)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub roles: JsonValue,
    pub created_at: DateTime,
}

#[derive(DeriveRelation, EnumIter, Debug)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
