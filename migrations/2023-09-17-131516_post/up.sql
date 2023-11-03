-- Your SQL goes here
CREATE TYPE     tipo_post AS ENUM ('video', 'texto', 'audio', 'html');
CREATE TYPE     lang AS ENUM ('pt', 'en', 'es', 'fr');
CREATE TABLE IF NOT EXISTS post (
                      id SERIAL PRIMARY KEY,
                      titulo VARCHAR(255) NOT NULL,
                      descripton VARCHAR(255) NOT NULL,
                      img TEXT DEFAULT '',
                      language lang NOT NULL,
                      categoria_id INT REFERENCES category(id) ON DELETE CASCADE,
                      total_views INT DEFAULT 0,
                      data_criacao TIMESTAMP DEFAULT current_timestamp,
                      tipo tipo_post NOT NULL,
                      conteudo  TEXT
);