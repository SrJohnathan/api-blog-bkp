-- Your SQL goes here
CREATE  TABLE IF NOT EXISTS matters
(
    id       SERIAL PRIMARY KEY,
    button VARCHAR(30)  NOT NULL,
    title VARCHAR(255) NOT NULL,
    content  TEXT,
    active   BOOLEAN      NOT NULL
);