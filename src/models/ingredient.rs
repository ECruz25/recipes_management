use crate::schema::ingredients;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Clone)]
#[table_name = "ingredients"]
pub struct Ingredient {
    pub id: String,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "ingredients"]
pub struct NewIngredient<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
