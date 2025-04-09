use crate::middleware::jwt::JWT;
use crate::middleware::response_models::NetworkResponse;
use chrono::Utc;
use entity::post::ActiveModel as PostModel;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        hello_world,
        create_post_placeholder,
        jebanko,
    ]
}

#[get("/")]
fn index() -> &'static str {
    "index page"
}

#[get("/hello/world")]
fn hello_world() -> &'static str {
    "hello world"
}

#[post("/post/placeholder")]
async fn create_post_placeholder(
    // connection: &State<DatabaseConnection>
)
 -> Result<status::Accepted<Json<String>>, Status> {
    // let connection = connection as &DatabaseConnection;
    //
    // let title = format!("post_title_{}", Utc::now().timestamp());
    //
    // let post_model = PostModel {
    //     id: NotSet,
    //     title: Set(title),
    //     content: Set(Some("dupa".to_string())),
    //     published: Set(false),
    //     created_at: Set(Utc::now().naive_local()),
    // };
    //
    // match post_model.insert(connection).await {
    //     Ok(result) => Ok(status::Accepted(Json(result))),
    //     Err(_) => Err(Status::InternalServerError),
    // }

    Ok(status::Accepted(Json("sranko w dupanko".to_string())))
}

#[get("/jebanko")]
fn jebanko(_jwt: JWT) -> Result<String, NetworkResponse> {
    Ok("ok".to_string())
}
