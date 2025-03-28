use rocket::serde::Serialize;
use sea_orm::prelude::*;

#[derive(DeriveEntityModel, Serialize, Debug, Clone)]
#[sea_orm(table_name = "post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub published: bool,
    pub created_at: DateTime,
}

#[derive(DeriveRelation, EnumIter, Debug)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
