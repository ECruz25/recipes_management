CREATE TABLE "schedules"
(
    "id"         CHAR(36)     NOT NULL
        CONSTRAINT schedule_pk
            PRIMARY KEY,
    "recipe_id"   CHAR(36)     NOT NULL
        CONSTRAINT "FK_schedules_recipes_recipe_id"
            REFERENCES "recipes",
    "date_of_food"       VARCHAR(255)         NOT NULL,
    "time_of_food" VARCHAR(255) NOT NULL
);
