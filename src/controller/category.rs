use rocket::{delete, get, post};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::mid::ConnectionManager;
use crate::models::models::Category;
use crate::models::new_models::NewCategory;
use crate::repository;



/// # Buscar todas as categorias
#[openapi(tag = "Category")]
#[get("/category")]
pub async  fn all(db:ConnectionManager<'_>) -> Result<status::Accepted<Json<Vec<Category>>>,status::BadRequest<String>>  {
   match  repository::cotegory::get_all_categories(db.0).await {
       Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
       Err(x) => Err( status::BadRequest(Some(x.to_string())))
   }
}


/// # Buscar uma categoria por ID
#[openapi(tag = "Category")]
#[get("/category/first/<id>")]
pub async  fn fisrt(db:ConnectionManager<'_>,id:i32) -> Result<status::Accepted<Json<Category>>,status::BadRequest<String>>  {
    match  repository::cotegory::get_category_by_id(db.0,id).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}

/// # Deletar uma categoria por ID
#[openapi(tag = "Category")]
#[delete("/category/<id>")]
pub async  fn delete(db:ConnectionManager<'_>,id:i32) -> Result<status::NoContent,status::BadRequest<String>>  {
    match  repository::cotegory::delete_category_by_id(db.0,id).await {
        Ok(_x) => Ok(status::NoContent),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Insere uma nova categoria
#[openapi(tag = "Category")]
#[post("/category", format = "application/json", data = "<task>")]
pub async  fn insert(db:ConnectionManager<'_>,task:Json<NewCategory>) -> Result<status::Created<Json<Category>>,status::BadRequest<String>>  {
    match  repository::cotegory::insert_category(db.0,&task).await {
        Ok(x) => Ok( status::Created::new("".to_string()).body(Json(x)) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


