table! {
    ingredients (id) {
        id -> Bpchar,
        name -> Varchar,
    }
}

table! {
    measurements (id) {
        id -> Bpchar,
        name -> Varchar,
        short_name -> Varchar,
    }
}

table! {
    products (id) {
        id -> Bpchar,
        ingredient_id -> Bpchar,
        measurement_id -> Bpchar,
        name -> Varchar,
        quantity -> Int4,
        price -> Numeric,
        brand -> Nullable<Varchar>,
    }
}

table! {
    recipe_ingredients (id) {
        id -> Bpchar,
        recipe_id -> Bpchar,
        ingredient_id -> Bpchar,
        measurement_id -> Bpchar,
        amount -> Varchar,
    }
}

table! {
    recipes (id) {
        id -> Bpchar,
        name -> Varchar,
        source -> Varchar,
    }
}

table! {
    schedules (id) {
        id -> Bpchar,
        recipe_id -> Bpchar,
        date_of_food -> Varchar,
        time_of_food -> Varchar,
        amount -> Int4,
    }
}

joinable!(recipe_ingredients -> ingredients (ingredient_id));
joinable!(recipe_ingredients -> measurements (measurement_id));
joinable!(recipe_ingredients -> recipes (recipe_id));
joinable!(schedules -> recipes (recipe_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    measurements,
    products,
    recipe_ingredients,
    recipes,
    schedules,
);
