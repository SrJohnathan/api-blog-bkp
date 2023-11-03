use rocket::{ delete, get, post};
use rocket::response::status;
use rocket::response::status::{BadRequest, Created};
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use crate::mid::{ConnectionManager, FormFile};
use crate::models::models::Files;
use crate::models::new_models::NewFiles;
use crate::repository;
use crate::s3::S3FileManager;


/// # Buscar todas as Arquivos
#[openapi(tag = "Files")]
#[get("/files")]
pub async fn all(db: ConnectionManager<'_>) -> Result<status::Accepted<Json<Vec<Files>>>, status::BadRequest<String>> {
    match repository::files::all(db.0).await {
        Ok(x) => Ok(status::Accepted(Some(Json(x)))),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}


/// # Deletar uma arquivo por ID
#[openapi(tag = "Files")]
#[delete("/files/<id>")]
pub async fn delete(db: ConnectionManager<'_>, id: i32) -> Result<status::NoContent, status::BadRequest<String>> {
    match repository::files::delete(db.0, id).await {
        Ok(x) => {
            let filemanager = S3FileManager::new(None, None, None, None);
            match filemanager.delete(x.name).await {
                Ok(_) => Ok(status::NoContent),
                Err(e) => Err(status::BadRequest(Some(e)))
            }
        }
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}


/// # Upload Arquivos
///  const formData = new FormData();
///
///  formData.append("media", file);
#[openapi(tag = "Files")]
#[post("/files/upload-media",format = "multipart/form-data", data = "<data>")]
pub async fn upload(db: ConnectionManager<'_>, data: FormFile,
) -> Result<Created<Json<Files>>, BadRequest<String>> {
    let  fi = NewFiles { name: data.file_name.unwrap() };
    let filemanager = S3FileManager::new(None, None, None, None);

    match filemanager.put_file_in_bucket_public(fi.name.clone(), data.data.0).await {
        Ok(_x) => {
            match crate::repository::files::insert(db.0,  fi.name).await {
                Ok(xx) => { Ok(Created::new("").body(Json(xx))) }
                Err(e) => Err(status::BadRequest(Some(e.to_string())))
            }
        }
        Err(e) => {
            Err(status::BadRequest(Some(e)))
        }
    }


}