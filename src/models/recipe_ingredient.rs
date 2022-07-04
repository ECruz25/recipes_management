use crate::schema::recipe_ingredients;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::ingredient::Ingredient;
use super::measurement::Measurement;
use super::recipe::Recipe;

#[derive(Identifiable, Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Associations)]
#[belongs_to(Ingredient)]
#[table_name = "recipe_ingredients"]
pub struct RecipeIngredient {
    pub id: String,
    pub recipe_id: String,
    pub ingredient_id: String,
    pub measurement_id: String,
    pub amount: String,
}

#[derive(Deserialize)]
pub struct RecipeMeasurement {
    pub id: String,
    pub ingredient_id: String,
    pub measurement_id: String,
    pub amount: f32,
}

#[derive(Deserialize)]
pub struct CreateRecipeIngredientDto {
    pub recipe_id: String,
    pub ingredients: Vec<RecipeMeasurement>,
}

#[derive(Insertable)]
#[table_name = "recipe_ingredients"]
pub struct NewRecipeIngredient<'a> {
    pub id: &'a str,
    pub recipe_id: &'a str,
    pub ingredient_id: &'a str,
    pub measurement_id: &'a str,
    pub amount: &'a str,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeIngredientComplete {
    pub id: String,
    pub ingredient_id: String,
    pub measurement_id: String,
    pub amount: String,
    pub recipe: Recipe,
    pub ingredient: Ingredient,
    pub measurement: Measurement,
}
