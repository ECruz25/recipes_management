CREATE TABLE "products"
(
    "id"           CHAR(36)     NOT NULL
        CONSTRAINT products_pk
            PRIMARY KEY,
    ingredient_id  CHAR(36)     NOT NULL
        CONSTRAINT "FK_recipe_ingredients_ingredients_ingredient_id"
            REFERENCES ingredients,
    measurement_id CHAR(36)     NOT NULL
        CONSTRAINT "FK_recipe_ingredients_measurements_measurement_id"
        REFERENCES  measurements,
    "name"         VARCHAR(255) NOT NULL,
    "quantity" INT NOT NULL DEFAULT 1,
    "price"        DECIMAL      NOT NULL DEFAULT 0,
    "brand"        VARCHAR      NULL
);