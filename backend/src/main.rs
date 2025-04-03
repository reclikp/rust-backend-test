#[macro_use]
extern crate rocket;

use crate::config::app_state::AppState;

mod config;
mod middleware;
mod controller;
mod repository;
mod service;

#[launch]
async fn rocket() -> _ {
    let state = AppState::new().await;

    rocket::build()
        .manage(state)
        .mount("/", controller::authentication::get_routes())
        .mount("/", controller::index::get_routes())
}
