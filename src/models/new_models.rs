use diesel::{AsChangeset, Insertable};
use diesel_derive_enum::DbEnum;
use rocket::data::Data;


use rocket::data::{ToByteUnit,FromData};
use rocket::form::{DataField, FromForm, FromFormField};
use rocket::fs::{FileName, TempFile};
use rocket::http::ContentType;
use rocket::http::ext::IntoCollection;
use rocket_okapi::OpenApiFromRequest;


use schemars::{JsonSchema,schema::Schema};
use schemars::gen::SchemaGenerator;



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

#[derive(Insertable, Debug,Deserialize,AsChangeset)]
#[table_name = "post"]
pub struct NewPost {
    pub titulo: String,
    pub categoria_id: i32,
    pub img: Option<String>,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}




#[derive(Debug,JsonSchema)]
pub struct FormNewPost {
    pub titulo:String,
    pub categoria_id: i32,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
    pub photo: DataFile,

}

#[derive(Debug)]
pub struct DataFile( pub Vec<u8>);

