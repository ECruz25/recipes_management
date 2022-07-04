// use crate::models::recipe::Recipe;
use crate::models::schedule::{NewSchedule, Schedule, ScheduleDto};
use crate::services::schedules_services::{create_schedule, get_schedules};
use crate::utils;
use rocket_contrib::json::Json;

#[rocket::get("/schedules")]
pub fn get() -> Json<Vec<ScheduleDto>> {
    let connection = utils::establish_connection();
    let results = get_schedules(&connection);
    Json(results)
}

#[rocket::post("/schedules", format = "json", data = "<schedule_input>")]
pub fn post(schedule_input: Json<NewSchedule>) -> Json<Schedule> {
    let connection = utils::establish_connection();
    let results = create_schedule(&schedule_input, &connection);
    Json(results)
}
