#[macro_use]
extern crate rocket;

mod config;
mod middleware;
mod controller;

#[launch]
async fn rocket() -> _ {
    let database = config::database::connect().await;

    rocket::build()
        // Temporary disabled because of changing to request guard
        // .attach(middleware::fairing::AuthenticationFairing)
        .manage(database)
        .mount("/", controller::authentication::get_routes())
        .mount("/", controller::index::get_routes())
}
