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
use controllers::planner_controller;
use controllers::recipes_controller;
use controllers::schedules_controller;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

fn main() {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![
                Method::Get,
                Method::Post,
                Method::Put,
                Method::Delete,
                Method::Patch,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);
    rocket::ignite()
        .attach(cors.to_cors().unwrap())
        .mount(
            "/",
            rocket::routes![
                ingredients_controller::get,
                ingredients_controller::post,
                recipes_controller::get,
                recipes_controller::post,
                recipes_controller::put,
                recipes_controller::get_by_id,
                schedules_controller::get,
                schedules_controller::post,
                measurements_controller::get,
                measurements_controller::post,
                planner_controller::post,
            ],
        )
        .launch();
}
