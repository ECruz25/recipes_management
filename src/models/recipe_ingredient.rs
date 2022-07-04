use crate::schema::recipe_ingredients;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::ingredient::Ingredient;
use super::measurement::Measurement;
use super::recipe::Recipe;

#[derive(
    Identifiable, Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Associations, Clone,
)]
#[belongs_to(Recipe, foreign_key = "recipe_id")]
#[belongs_to(Ingredient, foreign_key = "ingredient_id")]
#[belongs_to(Measurement, foreign_key = "measurement_id")]
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
pub struct RecipeIngredientDTO {
    pub id: String,
    pub ingredient_id: String,
    pub measurement_id: String,
    pub amount: String,
    pub recipe: Option<Recipe>,
    pub ingredient: Option<Ingredient>,
    pub measurement: Option<Measurement>,
}

impl RecipeIngredientDTO {
    pub fn build(
        recipe_ingredient: &RecipeIngredient,
        recipe: Option<Recipe>,
        ingredient: Option<Ingredient>,
        measurement: Option<Measurement>,
    ) -> RecipeIngredientDTO {
        RecipeIngredientDTO {
            id: recipe_ingredient.id.clone(),
            ingredient_id: recipe_ingredient.ingredient_id.clone(),
            measurement_id: recipe_ingredient.measurement_id.clone(),
            amount: recipe_ingredient.amount.clone(),
            recipe,
            ingredient,
            measurement,
        }
    }
}
