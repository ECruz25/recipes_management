use crate::schema::schedules;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::recipe::Recipe;

#[derive(Debug, PartialEq, Eq, Deserialize, Queryable, Serialize)]
pub struct Schedule {
    pub id: String,
    pub recipe_id: String,
    pub date_of_food: String,
    pub time_of_food: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "schedules"]
pub struct NewSchedule<'a> {
    pub id: &'a str,
    pub recipe_id: &'a str,
    pub date_of_food: &'a str,
    pub time_of_food: &'a str,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Queryable, Serialize)]
pub struct ScheduleWithRecipe {
    pub id: String,
    pub recipe_id: String,
    pub date_of_food: String,
    pub time_of_food: String,
    pub recipe: Recipe,
}
