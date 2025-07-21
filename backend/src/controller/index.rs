use crate::middleware::response_models::{ErrorResponse, Response};
use chrono::Utc;
use entity::post::ActiveModel as PostModel;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{Route, State};
use rocket::request::Outcome;
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use macros::require_role;
use crate::auth::jwt::JWT;

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
async fn create_post_placeholder() -> Result<status::Accepted<Json<String>>, Status> {
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
fn authenticated_content(_jwt: JWT) -> Result<String, ErrorResponse> {
    Ok("User has been authenticated".to_string())
}

#[require_role("admin", "cipsko")]
#[get("/admin-content")]
fn admin_content() -> Result<Response<String>, ErrorResponse> {
    // Err(ErrorResponse::new(Status::Forbidden, "Insufficient permission"))

    Ok(Response::new("Working as a shit".to_string()))
}
