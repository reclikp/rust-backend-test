#[macro_use]
extern crate rocket;

mod config;
mod middleware;
mod controller;
mod repository;
mod service;

#[launch]
async fn rocket() -> _ {
    let database = config::database::connect().await;

    rocket::build()
        .manage(database)
        .mount("/", controller::authentication::get_routes())
        .mount("/", controller::index::get_routes())
}
