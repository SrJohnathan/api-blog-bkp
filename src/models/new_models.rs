use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use diesel_derive_enum::DbEnum;










use schemars::{JsonSchema};




use serde_derive::{Deserialize, Serialize};


use crate::schema::{category,post,ads,files,settings};


#[derive(Debug,DbEnum,Deserialize, Serialize, JsonSchema,Clone)]
#[ExistingTypePath = "crate::schema::sql_types::TipoPost"]
pub enum TipoPost {
    Video,
    Texto,
    Audio,
    Html,
}

#[derive(Debug,DbEnum,Deserialize, Serialize, JsonSchema,Clone)]
#[ExistingTypePath = "crate::schema::sql_types::Lang"]
pub enum Language{
    Pt,
    En,
    Es,
    Fr
}




#[derive(Insertable, Debug,Deserialize,JsonSchema)]
#[diesel(table_name = category)]
pub struct NewCategory {
    pub name_pt: String,
    pub name_en: String,
    pub name_es: String,
    pub name_fr: String,
    pub active: bool,
}


#[derive(Insertable, Debug,Deserialize,JsonSchema)]
#[diesel(table_name = settings)]
pub struct NewSettings {
    pub name: String,
    pub data: Option<serde_json::Value>,

}


#[derive(Insertable, Debug,Deserialize,AsChangeset)]
#[diesel(table_name = post)]
pub struct NewPost {
    pub titulo: String,
    pub descripton: String,
    pub language : Language,
    pub categoria_id: i32,
    pub img: Option<String>,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}

#[derive(Debug,Deserialize,JsonSchema)]
pub struct NewPostIsert {
    pub titulo: String,
    pub description: String,
    pub language : Language,
    pub categoria_id: i32,
    pub img: Option<String>,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}


#[derive(Debug,JsonSchema)]
pub struct FormNewPost {
    pub titulo:String,
    pub description: String,
    pub language : Language,
    pub categoria_id: i32,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
    pub photo: DataFile,

}

#[derive(Queryable, Debug,Serialize,JsonSchema)]
pub struct PostWithCategory {
   pub id: i32,
   pub titulo: String,
   pub img: Option<String>,
   pub language: Language,
   pub description: String,
   pub categoria_id: Option<i32>,
   pub total_views: Option<i32>,
   pub data_criacao: Option<NaiveDateTime>,
   pub tipo: TipoPost,
   pub conteudo: Option<String>,
   pub name_category: String

}



#[derive(Insertable, Debug,Deserialize,JsonSchema)]
#[diesel(table_name = ads)]
pub struct NewAds {
    pub description: String,
    pub images: Option<Vec<i32>>,
    pub time: Option<i32>,
    pub url: Option<Vec<String>>,
    pub active: bool,
    pub alt: Option<serde_json::Value>,
}

#[derive( Debug,Deserialize,Serialize,JsonSchema)]
pub struct ResAds {
    pub id: i32,
    pub description: String,
    pub images: Vec<String>,
    pub time: Option<i32>,
    pub url: Vec<String>,
    pub active: bool,
    pub alt: Option<serde_json::Value>,
}

#[derive(Insertable, Debug,Deserialize)]
#[diesel(table_name = files)]
pub struct NewFiles {
    pub name: String,

}




#[derive(Debug)]
pub struct DataFile( pub Vec<u8>);

