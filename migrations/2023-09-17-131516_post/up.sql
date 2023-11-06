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


INSERT INTO public.post (id, titulo, descripton, img, language, categoria_id, total_views, data_criacao, tipo, conteudo) VALUES (1, 'Bem Vindo a STW', 'Olá sejá bem vindo a stw , a maior programa de intercambio de Portugal', 'stwww.png', 'pt', 2, 0, '2023-11-05 20:27:16.306966', 'texto', 'Olá sejá bem vindo a stw , a maior programa de intercambio de Portugal');
INSERT INTO public.post (id, titulo, descripton, img, language, categoria_id, total_views, data_criacao, tipo, conteudo) VALUES (2, 'Bem Vindo a STW 2', 'Olá sejá bem vindo a stw , a maior programa de intercambio de Portugal', 'stwww.png', 'pt', 2, 0, '2023-11-05 20:27:16.306966', 'texto', 'Olá sejá bem vindo a stw , a maior programa de intercambio de Portugal');
INSERT INTO public.post (id, titulo, descripton, img, language, categoria_id, total_views, data_criacao, tipo, conteudo) VALUES (3, 'Bem Vindo a STW 3', 'Olá sejá bem vindo a stw , a maior programa de intercambio de Portugal', 'stwww.png', 'pt', 2, 0, '2023-11-05 20:27:16.306966', 'texto', 'Olá sejá bem vindo a stw , a maior programa de intercambio de Portugal');