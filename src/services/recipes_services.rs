use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models::ingredient::Ingredient;
use crate::models::measurement::Measurement;
use crate::models::recipe;
use crate::models::recipe::GetRecipe;
use crate::models::recipe::RecipeFull;
use crate::models::recipe_ingredient::CreateRecipeIngredientDto;
use crate::models::recipe_ingredient::NewRecipeIngredient;
use crate::models::recipe_ingredient::RecipeIngredient;
use crate::models::recipe_ingredient::RecipeIngredientComplete;
use crate::models::recipe_ingredient::RecipeMeasurement;
use recipe::{NewRecipe, Recipe};

use super::ingredients_services;
use super::measurements_services;

pub fn get_recipes(conn: &PgConnection) -> Vec<Recipe> {
    use crate::schema::recipes::dsl::*;
    recipes.load::<Recipe>(conn).expect("No recipes")
}

pub fn get_recipe(recipe_id: &str, conn: &PgConnection) -> GetRecipe {
    use crate::schema::recipes::dsl::*;
    let result = recipes
        .filter(id.eq(recipe_id))
        .load::<Recipe>(conn)
        .expect("No recipes");
    GetRecipe {
        id: result[0].id.clone(),
        name: result[0].name.clone(),
        source: result[0].source.clone(),
    }
}

pub fn add_ingredient_to_recipe(
    data: NewRecipeIngredient,
    conn: &PgConnection,
) -> RecipeIngredient {
    use crate::schema::recipe_ingredients;
    diesel::insert_into(recipe_ingredients::table)
        .values(&data)
        .get_result(conn)
        .expect("Error saving recipe")
}

pub fn create_recipe(name: &str, source: &str, conn: &PgConnection) -> Recipe {
    use crate::schema::recipes;
    let id = Uuid::new_v4().to_string();
    let new_recipe: NewRecipe = NewRecipe {
        id: &id,
        name,
        source,
    };
    diesel::insert_into(recipes::table)
        .values(&new_recipe)
        .get_result(conn)
        .expect("Error saving recipe")
}

pub fn add_ingredients<'a>(data: &CreateRecipeIngredientDto, conn: &PgConnection) -> RecipeFull {
    let recipe = get_recipe(&data.recipe_id, &conn);
    let mut ingredients: Vec<RecipeIngredientComplete> = Vec::new();
    for recipe_ingr in data.ingredients.iter() {
        let (ingredient, measurement) = get_recipe_ingredient(&recipe_ingr, &conn);
        let id = Uuid::new_v4().to_string();
        let recipe_ingre: NewRecipeIngredient = NewRecipeIngredient {
            id: &id,
            recipe_id: &data.recipe_id,
            ingredient_id: &ingredient.id,
            measurement_id: &measurement.id,
            amount: &recipe_ingr.amount.to_string(),
        };
        let recip_ingredient = add_ingredient_to_recipe(recipe_ingre, &conn);
        ingredients.push(RecipeIngredientComplete {
            amount: recip_ingredient.amount,
            id,
            ingredient_id: ingredient.id.clone(),
            ingredient,
            measurement_id: measurement.id.clone(),
            measurement,
            recipe: Recipe {
                id: recipe.id.clone(),
                name: recipe.name.clone(),
                source: recipe.source.clone(),
            },
        });
    }
    RecipeFull {
        id: recipe.id.clone(),
        ingredients,
        name: recipe.name.clone(),
        source: recipe.source.clone(),
    }
}

pub fn get_recipe_ingredient(
    data: &RecipeMeasurement,
    conn: &PgConnection,
) -> (Ingredient, Measurement) {
    let ingredient = ingredients_services::get_ingredient(&data.ingredient_id, conn);
    let measurement = measurements_services::get_measurement(&data.measurement_id, conn);
    (
        Ingredient {
            id: ingredient.id,
            name: ingredient.name,
        },
        Measurement {
            id: measurement.id,
            name: measurement.name,
            short_name: measurement.short_name,
        },
    )
}
