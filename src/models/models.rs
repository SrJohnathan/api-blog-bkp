// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
use diesel::Queryable;
use diesel::sql_types::Json;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use crate::models::new_models::{Language, TipoPost};


#[derive(Queryable, Debug, Serialize, JsonSchema)]
pub struct Category {
    pub id: i32,
    pub name_url: String,
    pub name_pt: String,
    pub name_en: String,
    pub name_es: String,
    pub name_fr: String,
    pub active: bool,
}

#[derive(Queryable, Debug, Serialize, JsonSchema)]
pub struct Settings {
    pub id: i32,
    pub name: String,
    pub data: Option<serde_json::Value>,
   
}



#[derive(Queryable, Debug, Serialize, JsonSchema)]
pub struct Matters {
    pub id: i32,
    pub button:String,
    pub title:String,
   pub content :Option<String>,
   pub active: bool,

}

#[derive(Queryable, Debug, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = post)]
pub struct Post {
    pub id: i32,
    pub titulo: String,
    pub description: String,
    pub img: Option<String>,
    pub language: Language,
    pub categoria_id: Option<i32>,
    pub total_views: Option<i32>,
    pub data_criacao: Option<NaiveDateTime>,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}




#[derive(Queryable, Debug, Serialize, Deserialize, JsonSchema)]
#[diesel(table_name = ads)]
pub struct Ads {
    pub id: i32,
    pub description: String,
    pub images: Vec<Option<i32>>,
    pub time: Option<i32>,
    pub url: Vec<Option<String>>,
    pub active: bool,
    pub alt: Option<serde_json::Value>,
}


#[derive(Queryable, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Files {
    pub id: i32,
    pub name: String,

}

pub enum GetCategory<T, E> {
    ALL(E),
    ID(T),
}

pub enum AscDesc<T> {
    ASC(T),
    DESC(T),
}

