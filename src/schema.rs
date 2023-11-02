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
        #[max_length = 255]
        name -> Varchar,
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
    use diesel::sql_types::*;
    use super::sql_types::Lang;
    use super::sql_types::TipoPost;

    post (id) {
        id -> Int4,
        #[max_length = 255]
        titulo -> Varchar,
        img -> Nullable<Text>,
        language -> Lang,
        categoria_id -> Nullable<Int4>,
        total_views -> Nullable<Int4>,
        data_criacao -> Nullable<Timestamp>,
        tipo -> TipoPost,
        conteudo -> Nullable<Text>,
    }
}

diesel::joinable!(post -> category (categoria_id));

diesel::allow_tables_to_appear_in_same_query!(
    ads,
    category,
    files,
    post,
);
