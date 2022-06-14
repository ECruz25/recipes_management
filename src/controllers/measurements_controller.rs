use crate::models::measurement::{Measurement, NewMeasurement};
use crate::services::measurements_services::{create_measurement, get_measurements};
use crate::utils;
use rocket_contrib::json::Json;

#[rocket::get("/measurements")]
pub fn get() -> Json<Vec<Measurement>> {
    let connection = utils::establish_connection();
    let results = get_measurements(&connection);
    Json(results)
}

#[rocket::post("/measurements", format = "json", data = "<ingredient_input>")]
pub fn post(ingredient_input: Json<NewMeasurement>) -> Json<Measurement> {
    let connection = utils::establish_connection();
    let results = create_measurement(
        ingredient_input.name,
        ingredient_input.short_name,
        &connection,
    );
    Json(results)
}
