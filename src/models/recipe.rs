use crate::schema::recipes;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

use super::recipe_ingredient::RecipeIngredientComplete;

#[derive(Debug, PartialEq, Eq, Deserialize, Queryable, Serialize, Clone)]
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
#[derive(Serialize, Deserialize)]
pub struct RecipeFull {
    pub id: String,
    pub name: String,
    pub source: String,
    pub ingredients: Vec<RecipeIngredientComplete>,
}
