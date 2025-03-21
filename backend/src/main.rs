#[macro_use] extern crate rocket;

use rocket::serde::{ Deserialize,  Serialize, json::Json };
use rocket::{ State, response::status::Custom ,http::Status };
use tokio_postgres::{ Client, NoTls };
use rocket_cors::{ CorsOptions, AllowedOrigins };

struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

#[get("/")]
fn index() -> &'static str {
    return "index page";
}

#[get("/hello/world")]
fn hello_world() -> &'static str {
    return "hello world";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello_world])
}
