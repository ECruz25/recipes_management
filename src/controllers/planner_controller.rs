use crate::models::planner;
use crate::services::{ingredients_services, recipes_services, schedules_services};
use crate::utils;
use rocket_contrib::json::Json;

#[rocket::post("/planner")]
pub fn post() -> Json<planner::Planner> {
    let connection = utils::establish_connection();
    let schedules = schedules_services::get_schedules(&connection);
    let recipes = recipes_services::get_recipes_with_ingredients(&connection);
    let ingredients = ingredients_services::get_ingredients(&connection);
    let starting_date = "1";
    let planner_res = planner::Planner::build(
        &schedules,
        recipes.unwrap_or(vec![]).as_slice(),
        &ingredients,
        &starting_date,
        &starting_date,
    );
    Json(planner_res)
}
