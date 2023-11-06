-- Your SQL goes here
CREATE  TABLE IF NOT EXISTS category
(
    id       SERIAL PRIMARY KEY,
    name_url VARCHAR(30)  NOT NULL,
    name_pt  VARCHAR(255) NOT NULL,
    name_en  VARCHAR(255) NOT NULL,
    name_es  VARCHAR(255) NOT NULL,
    name_fr  VARCHAR(255) NOT NULL,
    active   BOOLEAN      NOT NULL
);


DO
$$
    BEGIN
        -- Oportunidades
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 1) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (1, 'oportunidades', 'Oportunidades', 'Opportunities', 'Oportunidades', 'Occasions', true);
        END IF;

        -- Noticias
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 2) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (2, 'noticias', 'Noticias', 'News', 'Noticias', 'Nouvelles', true);
        END IF;

        -- Mais Noticias
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 3) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (3, 'maisnoticias', 'Mais Noticias', 'More News', 'Más Noticias', 'Plus de nouvelles', true);
        END IF;

        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 4) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (4, 'servicos', 'Servicos', 'Services', 'Servicios', 'Services', true);
        END IF;

        -- Eventos
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 5) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (5, 'eventos', 'Eventos', 'Events', 'Eventos', 'Événements', true);
        END IF;

        -- Estude
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 6) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (6, 'estude', 'Estude', 'Study', 'Estudiar', 'Étude', true);
        END IF;

        -- Bolsas
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 7) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (7, 'bolsas', 'Bolsas', 'Scholarships', 'Becas', 'Bourses', true);
        END IF;

        -- Trabalhe
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 8) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (8, 'trabalhe', 'Trabalhe', 'Work', 'Trabajar', 'Travailler', true);
        END IF;

        -- Trabalhe Conosco
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 9) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (9, 'trabalheconosco', 'Trabalhe Conosco', 'Work With Us', 'Trabaja con nosotros',
                    'Travailler avec nous', true);
        END IF;

        -- Destinos
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 10) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (10, 'destinos', 'Destinos', 'Destinations', 'Destinos', 'Destinations', true);
        END IF;

        -- Universidades PT
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 11) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (11, 'universidadespt', 'Universidades PT', 'Universities PT', 'Universidades PT', 'Universités PT',
                    true);
        END IF;

        -- Academicos
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 12) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (12, 'academicos', 'Academicos', 'Academics', 'Académicos', 'Universitaires', true);
        END IF;

        -- Cursos
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 13) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (13, 'cursos', 'Cursos', 'Courses', 'Cursos', 'Cours', true);
        END IF;

        -- Programa de Integracao
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 14) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (14, 'programadeintegracao', 'Programa de Integracao', 'Integration Program',
                    'Programa de Integración', 'Programme d`intégration', true);
        END IF;

        -- Universidades pelo Mundo
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 15) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (15, 'universidadespelomundo', 'Universidades pelo Mundo', 'World Universities',
                    'Universidades del Mundo', 'Universités du Monde', true);
        END IF;

        -- Podcasts
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 16) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (16, 'podcasts', 'Podcasts', 'Podcasts', 'Podcasts', 'Podcasts', true);
        END IF;

        -- Top Students
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 17) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (17, 'topstudents', 'Top Students', 'Top Students', 'Mejores Estudiantes', 'Meilleurs étudiants',
                    true);
        END IF;

        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 18) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (18, 'tempo', 'Tempo', 'Weather', 'Tiempo', 'Météo', true);
        END IF;

        -- Ciencia e Inovacao
        IF NOT EXISTS (SELECT 1 FROM public.category WHERE id = 19) THEN
            INSERT INTO public.category (id, name_url, name_pt, name_en, name_es, name_fr, active)
            VALUES (19, 'cienciaeinovacao', 'Ciencia e Inovacao', 'Science and Innovation', 'Ciencia e Innovación',
                    'Science et Innovation', true);
        END IF;

    END
$$;