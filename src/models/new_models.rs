use diesel::{AsChangeset, Insertable};
use diesel_derive_enum::DbEnum;
use rocket_okapi::JsonSchema;
use serde_derive::{Deserialize, Serialize};

use crate::schema::{category,post};


#[derive(Debug,DbEnum,Deserialize, Serialize, JsonSchema)]
#[ExistingTypePath = "crate::schema::sql_types::TipoPost"]
#[serde(untagged)]
pub enum TipoPost {
    Fideo,
    Texto,
    Audio,
    Html,
}

#[derive(Insertable, Debug,Deserialize,JsonSchema)]
#[table_name = "category"]
pub struct NewCategory {
    pub name: String,
    pub active: bool,
}

#[derive(Insertable, Debug,Deserialize,JsonSchema,AsChangeset)]
#[table_name = "post"]
pub struct NewPost {
    pub titulo: String,
    pub categoria_id: i32,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}