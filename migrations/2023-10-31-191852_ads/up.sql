CREATE TABLE IF NOT EXISTS ads (
                                   id SERIAL PRIMARY KEY,
                                   description VARCHAR(255) NOT NULL,
                                   images INTEGER[] NOT NULL,
                                   time INTEGER DEFAULT 5,
                                   url TEXT[] NOT NULL,
                                   active BOOLEAN NOT NULL DEFAULT TRUE,
                                   alt JSON
);