use crate::middleware::jwt::JWT;
use crate::middleware::response_models::NetworkResponse;
use chrono::Utc;
use entity::post::ActiveModel as PostModel;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use auth_macros::require_role;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        create_post_placeholder,
        authenticated_content,
        admin_content,
    ]
}

#[get("/")]
fn index() -> &'static str {
    "index page"
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

#[get("/authenticated-content")]
fn authenticated_content(_jwt: JWT) -> Result<String, NetworkResponse> {
    Ok("User has been authenticated".to_string())
}

// #[require_role("admin")]
#[get("/admin-content")]
fn admin_content(_jwt: JWT) -> Result<String, NetworkResponse> {
    Ok("User has been authenticated".to_string())
}
