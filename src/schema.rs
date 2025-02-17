// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "lang"))]
    pub struct Lang;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tipo_post"))]
    pub struct TipoPost;
}

diesel::table! {
    ads (id) {
        id -> Int4,
        #[max_length = 255]
        description -> Varchar,
        images -> Array<Nullable<Int4>>,
        time -> Nullable<Int4>,
        url -> Array<Nullable<Text>>,
        active -> Bool,
        alt -> Nullable<Json>,
    }
}

diesel::table! {
    category (id) {
        id -> Int4,
        #[max_length = 30]
        name_url -> Varchar,
        #[max_length = 255]
        name_pt -> Varchar,
        #[max_length = 255]
        name_en -> Varchar,
        #[max_length = 255]
        name_es -> Varchar,
        #[max_length = 255]
        name_fr -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    matters (id) {
        id -> Int4,
        #[max_length = 30]
        button -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        content -> Nullable<Text>,
        active -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Lang;
    use super::sql_types::TipoPost;

    post (id) {
        id -> Int4,
        #[max_length = 255]
        titulo -> Varchar,
        #[max_length = 255]
        descripton -> Varchar,
        img -> Nullable<Text>,
        language -> Lang,
        categoria_id -> Nullable<Int4>,
        total_views -> Nullable<Int4>,
        data_criacao -> Nullable<Timestamp>,
        tipo -> TipoPost,
        conteudo -> Nullable<Text>,
    }
}

diesel::table! {
    settings (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        data -> Nullable<Jsonb>,
    }
}

diesel::joinable!(post -> category (categoria_id));

diesel::allow_tables_to_appear_in_same_query!(
    ads,
    category,
    files,
    matters,
    post,
    settings,
);
