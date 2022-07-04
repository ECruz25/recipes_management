use crate::schema::recipes;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::recipe_ingredient::{RecipeIngredient, RecipeIngredientDTO};

#[derive(Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Clone, Identifiable)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub source: String,
}
#[derive(Insertable)]
#[table_name = "recipes"]
pub struct NewRecipe<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub source: &'a str,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeDto {
    pub id: String,
    pub name: String,
    pub source: String,
    pub ingredients: Option<Vec<RecipeIngredientDTO>>,
}

impl RecipeDto {
    pub fn build(recipe: &Recipe, ingredients: &[RecipeIngredient]) -> RecipeDto {
        RecipeDto {
            id: recipe.id.clone(),
            name: recipe.name.clone(),
            source: recipe.source.clone(),
            ingredients: Some(
                ingredients
                    .into_iter()
                    .map(|ingredient| RecipeIngredientDTO::build(ingredient, None, None, None))
                    .collect(),
            ),
        }
    }
}
