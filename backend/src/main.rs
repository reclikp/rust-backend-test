mod database;

#[macro_use] extern crate rocket;

// use rocket::serde::{ Deserialize,  Serialize, json::Json };
// use rocket::{ State, response::status::Custom ,http::Status };
// use rocket_cors::{ CorsOptions, AllowedOrigins };

// struct User {
//     id: Option<i32>,
//     name: String,
//     email: String,
// }

#[get("/")]
fn index() -> &'static str {
    return "index page";
}

#[get("/hello/world")]
fn hello_world() -> &'static str {
    return "hello world";
}

#[launch]
async fn rocket() -> _ {
    let database = database::connect().await;

    rocket::build()
        .manage(database)
        .mount("/", routes![index, hello_world])
}
