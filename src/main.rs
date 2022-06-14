#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

mod controllers;
mod models;
mod schema;
mod services;
mod utils;

use controllers::ingredients_controller;
use controllers::measurements_controller;
use controllers::recipes_controller;
use controllers::schedules_controller;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            rocket::routes![
                ingredients_controller::get,
                ingredients_controller::post,
                recipes_controller::get,
                recipes_controller::post,
                recipes_controller::put,
                schedules_controller::get,
                schedules_controller::post,
                measurements_controller::get,
                measurements_controller::post
            ],
        )
        .launch();
}
