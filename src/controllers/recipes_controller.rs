use crate::models::recipe::{Recipe, RecipeFull};
use crate::models::recipe_ingredient::CreateRecipeIngredientDto;
use crate::services::recipes_services;
use crate::utils;
use rocket_contrib::json::Json;

#[rocket::get("/recipes")]
pub fn get() -> Json<Vec<Recipe>> {
    let connection = utils::establish_connection();
    let results = recipes_services::get_recipes(&connection);
    Json(results)
}

#[rocket::get("/recipes/<id>")]
pub fn get_by_id(id: String) -> Json<RecipeFull> {
    let connection = utils::establish_connection();
    let result = recipes_services::get_recipe_with_ingredients(&id, &connection);
    Json(result)
}

#[rocket::post("/recipes", format = "json", data = "<recipe_input>")]
pub fn post(recipe_input: Json<Recipe>) -> Json<Recipe> {
    let connection = utils::establish_connection();
    let results =
        recipes_services::create_recipe(&recipe_input.name, &recipe_input.source, &connection);
    Json(results)
}

#[rocket::put(
    "/recipes/ingredients",
    format = "json",
    data = "<recipe_ingredients_input>"
)]
pub fn put<'a>(recipe_ingredients_input: Json<CreateRecipeIngredientDto>) -> Json<RecipeFull> {
    let connection = utils::establish_connection();
    let results = recipes_services::add_ingredients(&recipe_ingredients_input, &connection);
    Json(results)
}
