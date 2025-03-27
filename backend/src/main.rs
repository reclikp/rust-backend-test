#[macro_use]
extern crate rocket;

mod config;
mod middleware;
mod controller;

use chrono::Utc;
use entity::post::{ActiveModel as PostModel};
use rocket::{State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "index page"
}

#[get("/hello/world")]
fn hello_world() -> &'static str {
    "hello world"
}

#[post("/post/placeholder")]
async fn create_post_placeholder(connection: &State<DatabaseConnection>) -> Result<status::Accepted<Json<entity::post::Model>>, Status> {
    let connection = connection as &DatabaseConnection;

    let title = format!("post_title_{}", Utc::now().timestamp());

    let post_model = PostModel {
        id: NotSet,
        title: Set(title),
        content: Set(Some("dupa".to_string())),
        published: Set(false),
        created_at: Set(Utc::now().naive_local()),
    };

    match post_model.insert(connection).await {
        Ok(result) => Ok(status::Accepted(Json(result))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
async fn rocket() -> _ {
    let database = config::database::connect().await;

    rocket::build()
        .attach(middleware::fairing::AuthenticationFairing)
        .manage(database)
        .mount("/", routes![index, hello_world, create_post_placeholder])
        .mount("/public", routes![controller::authentication::jebanko])
}
