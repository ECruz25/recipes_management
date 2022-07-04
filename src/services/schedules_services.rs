use crate::models::recipe::Recipe;
use crate::models::schedule;
use crate::schema::recipes;
use diesel::prelude::*;
use diesel::PgConnection;
use schedule::{NewSchedule, Schedule, ScheduleDto};
use uuid::Uuid;

pub fn get_schedules(conn: &PgConnection) -> Vec<ScheduleDto> {
    use crate::schema::schedules::dsl::*;
    let results: Vec<(Schedule, Recipe)> = schedules
        .inner_join(recipes::table)
        .load(conn)
        .expect("Could not load data");
    results
        .iter()
        .map(|(sch, rec)| ScheduleDto {
            id: sch.id.clone(),
            date_of_food: sch.date_of_food.clone(),
            recipe: Recipe {
                id: rec.id.clone(),
                name: rec.name.clone(),
                source: rec.source.clone(),
            },
            recipe_id: sch.recipe_id.clone(),
            time_of_food: sch.time_of_food.clone(),
            amount: sch.amount,
        })
        .collect()
}

pub fn create_schedule(data: &NewSchedule, conn: &PgConnection) -> Schedule {
    use crate::schema::schedules;
    let id = Uuid::new_v4().to_string();
    let new_recipe: NewSchedule = NewSchedule {
        id: &id,
        date_of_food: data.date_of_food,
        recipe_id: data.recipe_id,
        time_of_food: data.time_of_food,
        amount: data.amount,
    };
    diesel::insert_into(schedules::table)
        .values(&new_recipe)
        .get_result(conn)
        .expect("Error saving recipe")
}
