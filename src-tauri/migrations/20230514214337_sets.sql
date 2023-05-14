CREATE TABLE IF NOT EXISTS bricklink_sets
(
    id          VARCHAR(64) PRIMARY KEY NOT NULL,
    name        VARCHAR(250)            NOT NULL,
    category    INTEGER                 NOT NULL
);