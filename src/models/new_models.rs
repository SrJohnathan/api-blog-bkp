use diesel::{AsChangeset, Insertable};
use diesel_derive_enum::DbEnum;










use schemars::{JsonSchema};




use serde_derive::{Deserialize, Serialize};


use crate::schema::{category,post};


#[derive(Debug,DbEnum,Deserialize, Serialize, JsonSchema)]
#[ExistingTypePath = "crate::schema::sql_types::TipoPost"]
pub enum TipoPost {
    Video,
    Texto,
    Audio,
    Html,
}

#[derive(Debug,DbEnum,Deserialize, Serialize, JsonSchema)]
#[ExistingTypePath = "crate::schema::sql_types::Lang"]
pub enum Language{
    Pt,
 En, Es, Fr
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
    pub language : Language,
    pub categoria_id: i32,
    pub img: Option<String>,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
}




#[derive(Debug,JsonSchema)]
pub struct FormNewPost {
    pub titulo:String,
    pub language : Language,
    pub categoria_id: i32,
    pub tipo: TipoPost,
    pub conteudo: Option<String>,
    pub photo: DataFile,

}

#[derive(Debug)]
pub struct DataFile( pub Vec<u8>);

