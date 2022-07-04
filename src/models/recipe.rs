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

#[derive(Clone)]
pub struct GetRecipe {
    pub id: String,
    pub name: String,
    pub source: String,
}

impl GetRecipe {
    pub fn from_recipe(data: &Recipe) -> GetRecipe {
        GetRecipe {
            id: data.id.clone(),
            name: data.name.clone(),
            source: data.source.clone(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "recipes"]
pub struct NewRecipe<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub source: &'a str,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeDTO {
    pub id: String,
    pub name: String,
    pub source: String,
    pub ingredients: Option<Vec<RecipeIngredientDTO>>,
}

impl RecipeDTO {
    pub fn build(recipe: &Recipe, ingredients: &[RecipeIngredient]) -> RecipeDTO {
        RecipeDTO {
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
