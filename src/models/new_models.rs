use diesel::Insertable;
use rocket_okapi::JsonSchema;
use serde_derive::Deserialize;

use crate::schema::category;

#[derive(Insertable, Debug,Deserialize,JsonSchema)]
#[table_name = "category"]
pub struct NewCategory {
    pub name: String,
    pub active: bool,
}