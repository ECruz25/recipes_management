CREATE TABLE "ingredients"
(
    "id"     CHAR(36)     NOT NULL
        CONSTRAINT ingredients_pk
            PRIMARY KEY,
    "name"   VARCHAR(255) NOT NULL
);
