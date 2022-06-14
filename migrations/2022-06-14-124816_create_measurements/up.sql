CREATE TABLE "measurements"
(
    "id"     CHAR(36)     NOT NULL
        CONSTRAINT measurements_pk
            PRIMARY KEY,
    "name"   VARCHAR(255) NOT NULL,
    "short_name"   VARCHAR(255) NOT NULL
);