use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::ingredient::GetIngredient;
use crate::models::ingredient::{Ingredient, NewIngredient};

pub fn get_ingredients(conn: &PgConnection) -> Vec<Ingredient> {
    use crate::schema::ingredients::dsl::*;
    ingredients
        .load::<Ingredient>(conn)
        .expect("No ingredients")
}

pub fn create_ingredient(name: &str, conn: &PgConnection) -> Ingredient {
    use crate::schema::ingredients;
    let id = Uuid::new_v4().to_string();
    let new_ingredient: NewIngredient = NewIngredient { id: &id, name };
    diesel::insert_into(ingredients::table)
        .values(&new_ingredient)
        .get_result(conn)
        .expect("Error saving ingredient")
}

pub fn get_ingredient(ingredient_id: &str, conn: &PgConnection) -> GetIngredient {
    use crate::schema::ingredients::dsl::*;
    let result = ingredients
        .filter(id.eq(ingredient_id))
        .load::<Ingredient>(conn)
        .expect("No ingredients");
    GetIngredient {
        id: result[0].id.clone(),
        name: result[0].name.clone(),
    }
}
