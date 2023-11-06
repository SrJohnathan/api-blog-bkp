-- Your SQL goes here
CREATE TABLE settings (
                          id SERIAL PRIMARY KEY,
                          name VARCHAR(255) NOT NULL,
                          data jsonb

);

INSERT INTO public.settings (id, name, data) VALUES (1, 'mainpost', '{"ids": {"card1": 1, "card2": 3, "card3": 2}}');
