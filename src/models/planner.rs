use super::ingredient;
use super::measurement;
use super::recipe;
use super::schedule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct IngredientAmount {
    pub ingredient: ingredient::Ingredient,
    pub amount: f64,
    pub measurement: Option<measurement::Measurement>,
}

impl IngredientAmount {
    pub fn new(amount: f64, id: &str, ingredients: &[ingredient::Ingredient]) -> Self {
        let ingredient = ingredients
            .iter()
            .filter(|ingr| ingr.id == id)
            .map(|ingr| ingredient::Ingredient {
                id: ingr.id.clone(),
                name: ingr.name.clone(),
            })
            .nth(0)
            .unwrap();
        Self {
            ingredient,
            amount,
            measurement: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Planner {
    pub starting_date: String,
    pub end_date: String,
    pub ingredients: Vec<IngredientAmount>,
}

impl Planner {
    pub fn build(
        scheduled_recipes: &[schedule::ScheduleWithRecipe],
        existing_recipes: &[recipe::RecipeFull],
        ingredients: &[ingredient::Ingredient],
        starting_date: &str,
        end_date: &str,
    ) -> Self {
        let mut recipes_hs: HashMap<&str, f64> = HashMap::new();
        let mut ingredients_hs: HashMap<&str, f64> = HashMap::new();
        scheduled_recipes.into_iter().for_each(|schedule| {
            let recipe = recipes_hs.entry(&schedule.recipe_id).or_insert(0.0);
            *recipe += f64::from(schedule.amount);
        });

        recipes_hs.iter().for_each(|recipe| {
            existing_recipes
                .iter()
                .filter(|exis| exis.id == *recipe.0)
                .for_each(|recipe1| {
                    recipe1.ingredients.iter().for_each(|ingredient| {
                        let ingredint = ingredients_hs
                            .entry(&&ingredient.ingredient_id)
                            .or_insert(0.0);
                        let ingredient_amount: f64 = ingredient.amount.to_owned().parse().unwrap();
                        *ingredint += *recipe.1 * ingredient_amount;
                    });
                });
        });
        Planner {
            starting_date: String::from(starting_date),
            end_date: String::from(end_date),
            ingredients: ingredients_hs
                .into_iter()
                .map(|(id, amount)| IngredientAmount::new(amount, id, ingredients))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ingredient;
    use super::measurement;
    use super::recipe;
    use super::schedule;
    use super::IngredientAmount;
    use super::Planner;
    use crate::models::recipe_ingredient;
    #[test]
    fn it_works() {
        let ingredients = vec![ingredient::Ingredient {
            id: "i1".to_owned(),
            name: "ingredient 1".to_owned(),
        }];
        let mut scheduled_recipes: Vec<schedule::ScheduleWithRecipe> = Vec::new();
        scheduled_recipes.push(schedule::ScheduleWithRecipe {
            id: "s1".to_string(),
            recipe_id: "r1".to_string(),
            amount: 1,
            date_of_food: "2020-01-01".to_string(),
            time_of_food: "Lunch".to_string(),
            recipe: recipe::Recipe {
                source: "".to_string(),
                id: "r1".to_string(),
                name: "recipe 1".to_string(),
            },
        });
        scheduled_recipes.push(schedule::ScheduleWithRecipe {
            id: "s2".to_string(),
            recipe_id: "r1".to_string(),
            amount: 2,
            date_of_food: "2020-01-02".to_string(),
            time_of_food: "Lunch".to_string(),
            recipe: recipe::Recipe {
                source: "".to_string(),
                id: "r1".to_string(),
                name: "recipe 1".to_string(),
            },
        });
        let mut existing_recipes = Vec::new();
        existing_recipes.push(recipe::RecipeFull {
            source: "".to_string(),
            id: "r1".to_string(),
            name: "recipe 1".to_string(),
            ingredients: vec![recipe_ingredient::RecipeIngredientComplete {
                id: "ri1".to_string(),
                ingredient_id: "i1".to_string(),
                measurement_id: "m1".to_string(),
                amount: "1".to_string(),
                ingredient: ingredient::Ingredient {
                    id: "i1".to_string(),
                    name: "ingredient 1".to_string(),
                },
                measurement: measurement::Measurement {
                    id: "m1".to_string(),
                    name: "measurement 1".to_string(),
                    short_name: "m1".to_string(),
                },
                recipe: recipe::Recipe {
                    source: "".to_string(),
                    id: "r1".to_string(),
                    name: "recipe 1".to_string(),
                },
            }],
        });
        let expected = Planner {
            starting_date: "2020-01-01".to_string(),
            end_date: "2020-01-02".to_string(),
            ingredients: vec![IngredientAmount {
                ingredient: ingredient::Ingredient {
                    id: "1".to_string(),
                    name: "ingredient 1".to_string(),
                },
                measurement: Some(measurement::Measurement {
                    id: "m1".to_string(),
                    name: "measurement 1".to_string(),
                    short_name: "m1".to_string(),
                }),
                amount: 3.0,
            }],
        }
        .ingredients
        .first()
        .to_owned()
        .unwrap()
        .amount;

        let result = Planner::build(
            &scheduled_recipes,
            &existing_recipes,
            &ingredients,
            "2020-01-01",
            "2020-01-02",
        )
        .ingredients
        .first()
        .unwrap()
        .amount;
        assert_eq!(result, expected);
    }
}
