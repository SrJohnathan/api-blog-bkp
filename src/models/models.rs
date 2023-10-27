// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
use diesel::Queryable;
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use crate::models::new_models::{Language, TipoPost};


#[derive(Queryable, Debug,Serialize,JsonSchema)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub active: bool,
}

#[derive(Queryable, Debug,Serialize,Deserialize,JsonSchema)]
pub struct Post {
    pub id: i32,
    pub titulo: String,
    pub img: Option<String>,
    pub language : Language,
    pub categoria_id: Option<i32>,
    pub total_views: Option<i32>,
    pub data_criacao: Option<NaiveDateTime>,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}

pub enum GetCategory<T,E>{
    ALL(E),
    ID(T)
}

pub enum AscDesc <T>{
    ASC(T),
    DESC(T)
}

