use crate::models::ingredient::{Ingredient, NewIngredient};
use crate::services::ingredients_services::{create_ingredient, get_ingredients};
use crate::utils;
use rocket_contrib::json::Json;

#[rocket::get("/ingredients")]
pub fn get() -> Json<Vec<Ingredient>> {
    let connection = utils::establish_connection();
    let results = get_ingredients(&connection);
    Json(results)
}

#[rocket::post("/ingredients", format = "json", data = "<ingredient_input>")]
pub fn post(ingredient_input: Json<NewIngredient>) -> Json<Ingredient> {
    let connection = utils::establish_connection();
    let results = create_ingredient(ingredient_input.name, &connection);
    Json(results)
}
