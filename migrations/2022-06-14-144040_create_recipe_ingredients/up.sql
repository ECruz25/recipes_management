CREATE TABLE "recipe_ingredients"
(
    "id"     CHAR(36)     NOT NULL
        CONSTRAINT recipe_ingredients_pk
            PRIMARY KEY,
    "recipe_id"   CHAR(36)     NOT NULL
        CONSTRAINT "FK_recipe_ingredients_recipes_recipe_id"
            REFERENCES "recipes",
    "ingredient_id"   CHAR(36)     NOT NULL
        CONSTRAINT "FK_recipe_ingredients_ingredients_ingredient_id"
            REFERENCES "ingredients",
    "measurement_id"   CHAR(36)     NOT NULL
        CONSTRAINT "FK_recipe_ingredients_measurements_measurement_id"
            REFERENCES "measurements",
    "amount"   VARCHAR(255) NOT NULL
);
