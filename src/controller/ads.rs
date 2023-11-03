use rocket::{delete, get, post};
use rocket::response::status;
use rocket::response::status::{Accepted, BadRequest};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::mid::ConnectionManager;
use crate::models::models::Ads;
use crate::models::new_models::{NewAds, ResAds};
use crate::repository;

/// # Buscar todas as categorias
#[openapi(tag = "Ads")]
#[get("/ads")]
pub async  fn all(db:ConnectionManager<'_>) -> Result<status::Accepted<Json<Vec<Ads>>>,status::BadRequest<String>>  {
    match  repository::ads::get_all_categories(db.0).await {
        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Buscar uma categoria por ID
#[openapi(tag = "Ads")]
#[get("/ads/first/<id>")]
pub async  fn fisrt(db:ConnectionManager<'_>, id:i32) -> Result<Accepted<Json<ResAds>>, BadRequest<String>> {
    match  repository::ads::get_ads_by_id(db.0,id).await {


        Ok(x) => Ok( status::Accepted(Some(Json(x))) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}

/// # Deletar uma categoria por ID
#[openapi(tag = "Ads")]
#[delete("/ads/<id>")]
pub async  fn delete(db:ConnectionManager<'_>,id:i32) -> Result<status::NoContent,status::BadRequest<String>>  {
    match  repository::ads::delete_ads_by_id(db.0,id).await {
        Ok(_x) => Ok(status::NoContent),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}


/// # Insere uma nova categoria
#[openapi(tag = "Ads")]
#[post("/ads", format = "application/json", data = "<task>")]
pub async  fn insert(db:ConnectionManager<'_>,task:Json<NewAds>) -> Result<status::Created<Json<Ads>>,status::BadRequest<String>>  {
    match  repository::ads::insert_ads(db.0,&task).await {
        Ok(x) => Ok( status::Created::new("".to_string()).body(Json(x)) ),
        Err(x) => Err( status::BadRequest(Some(x.to_string())))
    }
}