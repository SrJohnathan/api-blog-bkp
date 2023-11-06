use rocket::{delete, get, post};
use rocket::response::status;
use rocket::response::status::{Accepted, BadRequest, NoContent};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::mid::ConnectionManager;
use crate::models::models::Settings;
use crate::models::new_models::NewSettings;
use crate::repository;

/// # Buscar todas as categorias
#[openapi(tag = "Settings")]
#[get("/settings")]
pub async  fn all(db:ConnectionManager<'_>) -> Result<Accepted<Json<Vec<Settings>>>, BadRequest<String>> {
    match  repository::settings::get_all(db.0).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Buscar uma categoria por ID
#[openapi(tag = "Settings")]
#[get("/settings/first/<id>")]
pub async  fn fisrt(db:ConnectionManager<'_>, id:i32) -> Result<Accepted<Json<usize>>, BadRequest<String>> {
    match  repository::settings::delete_by_id(db.0,id).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}

/// # Buscar uma categoria por ID
#[openapi(tag = "Settings")]
#[get("/settings/name/<name>")]
pub async  fn fisrt_name(db:ConnectionManager<'_>, name:String) -> Result<Accepted<Json<Settings>>, BadRequest<String>> {
    match  repository::settings::get_by_name(db.0,name).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Deletar uma categoria por ID
#[openapi(tag = "Settings")]
#[delete("/settings/<id>")]
pub async  fn delete(db:ConnectionManager<'_>, id:i32) -> Result<NoContent, BadRequest<String>> {
    match  repository::settings::delete_by_id(db.0,id).await {
        Ok(_x) => Ok(status::NoContent),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Insere uma nova categoria
#[openapi(tag = "Settings")]
#[post("/settings", format = "application/json", data = "<task>")]
pub async  fn insert(db:ConnectionManager<'_>,task:Json<NewSettings>) -> Result<status::Created<Json<Settings>>,status::BadRequest<String>>    {
    match  repository::settings::insert(db.0,&task).await {
        Ok(x) => Ok( status::Created::new("".to_string()).body(Json(x)) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}