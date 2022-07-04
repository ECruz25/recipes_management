use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use crate::models;
use crate::models::recipe_ingredient::{
    CreateRecipeIngredientDto, NewRecipeIngredient, RecipeIngredient,
};

use super::ingredients_services;
use super::measurements_services;

pub fn get_recipe(
    recipe_id: &str,
    conn: &PgConnection,
) -> Result<models::recipe::Recipe, &'static str> {
    use crate::schema::recipes::dsl::*;
    let result = recipes
        .filter(id.eq(recipe_id))
        .load::<models::recipe::Recipe>(conn)
        .unwrap();
    let first = result.first();
    match first {
        Some(recipe) => Ok(recipe.clone()),
        None => Err("No recipe found"),
    }
}

pub fn get_recipes(
    conn: &PgConnection,
) -> Result<Vec<models::recipe::Recipe>, diesel::result::Error> {
    use crate::schema::recipes::dsl::*;
    recipes.load::<models::recipe::Recipe>(conn)
}

pub fn get_recipes_with_ingredients(
    conn: &PgConnection,
) -> Result<Vec<models::recipe::RecipeDto>, &'static str> {
    use crate::schema::recipes::dsl::*;
    let recipes_vec = recipes
        .load::<models::recipe::Recipe>(conn)
        .expect("Error loading user");
    let ingredients_vec = RecipeIngredient::belonging_to(&recipes_vec)
        .load::<RecipeIngredient>(conn)
        .expect("Error loading ingredients");
    let grouped_ingredients = ingredients_vec.grouped_by(&recipes_vec);
    let result: Vec<(
        models::recipe::Recipe,
        Vec<models::recipe_ingredient::RecipeIngredient>,
    )> = recipes_vec.into_iter().zip(grouped_ingredients).collect();
    let result = result
        .into_iter()
        .map(|(recipe, ingredients)| models::recipe::RecipeDto::build(&recipe, &ingredients))
        .collect();
    Ok(result)
}

// pub fn get_recipe_with_ingredients(r_id: &str, conn: &PgConnection) -> models::recipe::RecipeDTO {
//     use crate::schema::recipe_ingredients::dsl::*;
//     let result: Vec<(
//         models::recipe_ingredient::RecipeIngredient,
//         models::recipe::Recipe,
//         models::ingredient::Ingredient,
//         models::measurement::Measurement,
//     )> = recipe_ingredients
//         .filter(recipe_id.eq(r_id))
//         .inner_join(schema::recipes::table)
//         .inner_join(schema::ingredients::table)
//         .inner_join(schema::measurements::table)
//         .load(conn)
//         .expect("asd");
//     let recipe_ingredients_vec: Vec<RecipeIngredientComplete> = result
//         .into_iter()
//         .map(
//             |(recipe_ingredient, recip, ingredient, measurement)| RecipeIngredientComplete {
//                 amount: recipe_ingredient.amount,
//                 id: recipe_ingredient.id.clone(),
//                 ingredient_id: ingredient.id.clone(),
//                 measurement_id: measurement.id.clone(),
//                 ingredient,
//                 measurement,
//                 recipe: recip,
//             },
//         )
//         .collect();
//     match recipe_ingredients_vec.len() {
//         0 => {
//             let recipe = get_recipe(r_id, conn).expect("Not found");
//             models::recipe::RecipeDTO {
//                 id: String::from(r_id),
//                 ingredients: Vec::new(),
//                 name: recipe.name,
//                 source: recipe.source,
//             }
//         }
//         _ => {
//             let recipe1 = &recipe_ingredients_vec.first().clone().unwrap().recipe;
//             let recipe = models::recipe::GetRecipe::from_recipe(&recipe1);
//             models::recipe::RecipeDTO {
//                 id: recipe.id.clone(),
//                 ingredients: recipe_ingredients_vec,
//                 name: recipe.name.clone(),
//                 source: recipe.source.clone(),
//             }
//         }
//     }
// }

pub fn add_ingredient_to_recipe(
    data: NewRecipeIngredient,
    conn: &PgConnection,
) -> Result<models::recipe_ingredient::RecipeIngredient, diesel::result::Error> {
    use crate::schema::recipe_ingredients;
    diesel::insert_into(recipe_ingredients::table)
        .values(&data)
        .get_result(conn)
}

pub fn update_recipe_ingredient(
    rec_ing_id: &str,
    new_amount: &str,
    measure_id: &str,
    conn: &PgConnection,
) {
    use crate::schema::recipe_ingredients::dsl::*;
    let target = recipe_ingredients.filter(id.eq(rec_ing_id));
    let _ = diesel::update(target)
        .set((amount.eq(new_amount), measurement_id.eq(measure_id)))
        .execute(conn);
}

pub fn create_recipe(
    name: &str,
    source: &str,
    conn: &PgConnection,
) -> Result<models::recipe::Recipe, diesel::result::Error> {
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
) -> Result<models::recipe::RecipeDto, &'static str> {
    let recipe = get_recipe_with_ingredients(&data.recipe_id, &conn)?;
    for recipe_ingr in data.ingredients.iter() {
        let ingredient = ingredients_services::get_ingredient(&recipe_ingr.ingredient_id, conn)?;
        let measurement =
            measurements_services::get_measurement(&recipe_ingr.measurement_id, conn)?;
        let recipe_ingredient_exists = get_recipe_ingredient(&recipe.id, &ingredient.id, conn);
        if recipe_ingredient_exists != "" {
            update_recipe_ingredient(
                &recipe_ingr.id,
                &recipe_ingr.amount.to_string(),
                &measurement.id,
                conn,
            );
            continue;
        }
        let id = Uuid::new_v4().to_string();
        let recipe_ingre = NewRecipeIngredient {
            id: &id,
            recipe_id: &data.recipe_id,
            ingredient_id: &ingredient.id,
            measurement_id: &measurement.id,
            amount: &recipe_ingr.amount.to_string(),
        };
        let _ = add_ingredient_to_recipe(recipe_ingre, &conn);
    }
    get_recipe_with_ingredients(&data.recipe_id, conn)
}

// pub fn get_asnasd(
//     data: &RecipeMeasurement,
//     conn: &PgConnection,
// ) -> Result<RecipeIngredient, &'static str> {
//     let ingredient = ingredients_services::get_ingredient(&data.ingredient_id, conn)?;
//     let measurement = measurements_services::get_measurement(&data.measurement_id, conn)?;
//     let recipe_ingredient = RecipeIngredient::belonging_to(&ingredient).first(conn);
//     match recipe_ingredient {
//         Ok(recipe_ingredient) => Ok(recipe_ingredient),
//         Err(_) => Err("- not found"),
//     }
// }

pub fn get_recipe_with_ingredients(
    recipe_id: &str,
    conn: &PgConnection,
) -> Result<models::recipe::RecipeDto, &'static str> {
    let recipe = get_recipe(recipe_id, conn)?;
    let ingredients = RecipeIngredient::belonging_to(&recipe)
        .load::<RecipeIngredient>(conn)
        .expect("Error loading ingredients");
    Ok(models::recipe::RecipeDto::build(&recipe, &ingredients))
}
