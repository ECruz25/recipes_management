use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models;
use crate::models::recipe_ingredient::CreateRecipeIngredientDto;
use crate::models::recipe_ingredient::NewRecipeIngredient;
use crate::models::recipe_ingredient::RecipeIngredient;
use crate::models::recipe_ingredient::RecipeIngredientComplete;
use crate::models::recipe_ingredient::RecipeMeasurement;

use super::ingredients_services;
use super::measurements_services;
use crate::schema;

pub fn get_recipe(recipe_id: &str, conn: &PgConnection) -> models::recipe::GetRecipe {
    use crate::schema::recipes::dsl::*;
    let result = recipes
        .filter(id.eq(recipe_id))
        .load::<models::recipe::Recipe>(conn)
        .expect("No recipes");
    models::recipe::GetRecipe {
        id: result[0].id.clone(),
        name: result[0].name.clone(),
        source: result[0].source.clone(),
    }
}

pub fn get_recipes(conn: &PgConnection) -> Vec<models::recipe::Recipe> {
    use crate::schema::recipes::dsl::*;
    recipes
        .load::<models::recipe::Recipe>(conn)
        .expect("No recipes")
}

pub fn get_recipe_with_ingredients(r_id: &str, conn: &PgConnection) -> models::recipe::RecipeFull {
    use crate::schema::recipe_ingredients::dsl::*;
    let result: Vec<(
        models::recipe_ingredient::RecipeIngredient,
        models::recipe::Recipe,
        models::ingredient::Ingredient,
        models::measurement::Measurement,
    )> = recipe_ingredients
        .filter(recipe_id.eq(r_id))
        .inner_join(schema::recipes::table)
        .inner_join(schema::ingredients::table)
        .inner_join(schema::measurements::table)
        .load(conn)
        .expect("asd");
    let recipe_ingredients_vec: Vec<RecipeIngredientComplete> = result
        .into_iter()
        .map(
            |(recipe_ingredient, recip, ingredient, measurement)| RecipeIngredientComplete {
                amount: recipe_ingredient.amount,
                id: recipe_ingredient.id.clone(),
                ingredient_id: ingredient.id.clone(),
                measurement_id: measurement.id.clone(),
                ingredient,
                measurement,
                recipe: recip,
            },
        )
        .collect();
    if recipe_ingredients_vec.len() == 0 {
        // let recipe = result.into_iter().next().unwrap().1; //.first().unwrap().1.clone();
        let recipe = get_recipe(r_id, conn);
        return models::recipe::RecipeFull {
            id: String::from(r_id),
            ingredients: Vec::new(),
            name: recipe.name,
            source: recipe.source,
        };
        // return get_recipe(r_id, conn);
    }
    let recipe1 = &recipe_ingredients_vec.first().clone().unwrap().recipe;
    let recipe = models::recipe::GetRecipe::from_recipe(&recipe1);
    models::recipe::RecipeFull {
        id: recipe.id.clone(),
        ingredients: recipe_ingredients_vec,
        name: recipe.name.clone(),
        source: recipe.source.clone(),
    }
}

pub fn add_ingredient_to_recipe(
    data: NewRecipeIngredient,
    conn: &PgConnection,
) -> models::recipe_ingredient::RecipeIngredient {
    use crate::schema::recipe_ingredients;
    diesel::insert_into(recipe_ingredients::table)
        .values(&data)
        .get_result(conn)
        .expect("Error saving recipe")
}

pub fn update_recipe_ingredient(
    rec_ing_id: &str,
    new_amount: &str,
    measure_id: &str,
    conn: &PgConnection,
) -> models::recipe_ingredient::RecipeIngredient {
    use crate::schema::recipe_ingredients::dsl::*;
    let target = recipe_ingredients.filter(id.eq(rec_ing_id));
    diesel::update(target)
        .set((amount.eq(new_amount), measurement_id.eq(measure_id)))
        .get_result(conn)
        .unwrap()
}

pub fn create_recipe(name: &str, source: &str, conn: &PgConnection) -> models::recipe::Recipe {
    use crate::schema::recipes;
    let id = Uuid::new_v4().to_string();
    let new_recipe: models::recipe::NewRecipe = models::recipe::NewRecipe {
        id: &id,
        name,
        source,
    };
    diesel::insert_into(recipes::table)
        .values(&new_recipe)
        .get_result(conn)
        .expect("Error saving recipe")
}

pub fn get_recipe_ingredient(reci_id: &str, ingre_id: &str, conn: &PgConnection) -> String {
    use crate::schema::recipe_ingredients::dsl::*;
    let r1: models::recipe_ingredient::RecipeIngredient = recipe_ingredients
        .filter(ingredient_id.eq(ingre_id))
        .filter(recipe_id.eq(reci_id))
        .first(conn)
        .unwrap_or_else(|_| {
            return RecipeIngredient {
                amount: String::from(":"),
                id: String::from(""),
                ingredient_id: String::from(""),
                measurement_id: String::from(""),
                recipe_id: String::from(""),
            };
        });
    r1.id.clone()
}

pub fn add_ingredients<'a>(
    data: &CreateRecipeIngredientDto,
    conn: &PgConnection,
) -> models::recipe::RecipeFull {
    let recipe = get_recipe_with_ingredients(&data.recipe_id, &conn);
    let mut ingredients: Vec<RecipeIngredientComplete> = Vec::new();
    for recipe_ingr in data.ingredients.iter() {
        let (ingredient, measurement) = get_ingredient_measurement(&recipe_ingr, &conn);
        let recipe_ingredient_exists = get_recipe_ingredient(&recipe.id, &ingredient.id, conn);
        if recipe_ingredient_exists != "" {
            let recip_ingredient123 = update_recipe_ingredient(
                &recipe_ingr.id,
                &recipe_ingr.amount.to_string(),
                &measurement.id,
                conn,
            );
            ingredients.push(RecipeIngredientComplete {
                amount: recip_ingredient123.amount,
                id: recipe_ingr.id.clone(),
                ingredient_id: ingredient.id.clone(),
                ingredient,
                measurement_id: measurement.id.clone(),
                measurement,
                recipe: models::recipe::Recipe {
                    id: recipe.id.clone(),
                    name: recipe.name.clone(),
                    source: recipe.source.clone(),
                },
            });
            continue;
        }
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
            recipe: models::recipe::Recipe {
                id: recipe.id.clone(),
                name: recipe.name.clone(),
                source: recipe.source.clone(),
            },
        });
    }
    models::recipe::RecipeFull {
        id: recipe.id.clone(),
        ingredients,
        name: recipe.name.clone(),
        source: recipe.source.clone(),
    }
}

pub fn get_ingredient_measurement(
    data: &RecipeMeasurement,
    conn: &PgConnection,
) -> (
    models::ingredient::Ingredient,
    models::measurement::Measurement,
) {
    let ingredient = ingredients_services::get_ingredient(&data.ingredient_id, conn);
    let measurement = measurements_services::get_measurement(&data.measurement_id, conn);
    (
        models::ingredient::Ingredient {
            id: ingredient.id,
            name: ingredient.name,
        },
        models::measurement::Measurement {
            id: measurement.id,
            name: measurement.name,
            short_name: measurement.short_name,
        },
    )
}
