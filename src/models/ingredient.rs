use crate::schema::ingredients;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Clone)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetIngredient {
    pub id: String,
    pub name: String,
}
