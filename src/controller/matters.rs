use rocket::{delete, get, post};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::mid::ConnectionManager;
use crate::models::models::Matters;
use crate::models::new_models::NewMatters;
use crate::repository;

/// # Buscar todas as categorias
#[openapi(tag = "Matters")]
#[get("/matters")]
pub async  fn all(db:ConnectionManager<'_>) -> Result<status::Accepted<Json<Vec<Matters>>>,status::BadRequest<String>>  {
    match  repository::matters::get_all(db.0).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Buscar uma categoria por ID
#[openapi(tag = "Matters")]
#[get("/matters/first/<id>")]
pub async  fn fisrt(db:ConnectionManager<'_>,id:i32) -> Result<status::Accepted<Json<Matters>>,status::BadRequest<String>>  {
    match  repository::matters::get_by_id(db.0,id).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}

/// # Buscar uma categoria por ID
#[openapi(tag = "Matters")]
#[get("/matters/button/<name_url>")]
pub async  fn fisrt_name(db:ConnectionManager<'_>,name_url:String) -> Result<status::Accepted<Json<Matters>>,status::BadRequest<String>>  {
    match  repository::matters::get_by_button(db.0,name_url).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Deletar uma categoria por ID
#[openapi(tag = "Matters")]
#[delete("/matters/<id>")]
pub async  fn delete(db:ConnectionManager<'_>,id:i32) -> Result<status::NoContent,status::BadRequest<String>>  {
    match  repository::matters::delete_by_id(db.0,id).await {
        Ok(_x) => Ok(status::NoContent),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Insere uma nova categoria
#[openapi(tag = "Matters")]
#[post("/matters", format = "application/json", data = "<task>")]
pub async  fn insert(db:ConnectionManager<'_>,task:Json<NewMatters>) -> Result<status::Created<Json<Matters>>,status::BadRequest<String>>  {
    match  repository::matters::insert(db.0,&task).await {
        Ok(x) => Ok( status::Created::new("".to_string()).body(Json(x)) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}