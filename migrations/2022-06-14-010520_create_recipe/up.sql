CREATE TABLE "recipes"
(
    "id"     CHAR(36)     NOT NULL
        CONSTRAINT recipe_pk
            PRIMARY KEY,
    "name"   VARCHAR(255) NOT NULL,
    "source" VARCHAR(255) NOT NULL
);
