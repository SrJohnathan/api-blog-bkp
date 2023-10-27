use rocket::{delete, get, post, put};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use uuid::Uuid;

use crate::mid::ConnectionManager;
use crate::models::models::Post;
use crate::models::new_models::{FormNewPost, NewPost};
use crate::repository;
use crate::s3::S3FileManager;


/// # Buscar todas as Posts
#[openapi(tag = "Post")]
#[get("/post")]
pub async fn all(db: ConnectionManager<'_>) -> Result<status::Accepted<Json<Vec<Post>>>, status::BadRequest<String>> {
    match repository::post::get_all_posts(db.0).await {
        Ok(x) => Ok(status::Accepted(Some(Json(x)))),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}

/// # a chamar ,adiciona uma view no  Post
#[openapi(tag = "Post")]
#[get("/post/insert_view/<id>")]
pub async fn view(db: ConnectionManager<'_>, id: i32) -> Result<status::Accepted<Json<Post>>, status::BadRequest<String>> {
    match repository::post::increment_total_views(db.0, id).await {
        Ok(x) => Ok(status::Accepted(Some(Json(x)))),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}


/// # Buscar uma Post por ID
#[openapi(tag = "Post")]
#[get("/post/<id>")]
pub async fn fisrt(db: ConnectionManager<'_>, id: i32) -> Result<status::Accepted<Json<Post>>, status::BadRequest<String>> {
    match repository::post::get_post_by_id(db.0, id).await {
        Ok(x) => Ok(status::Accepted(Some(Json(x)))),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}


/// # Buscar uma Posts por Categoria
#[openapi(tag = "Post")]
#[get("/post/category/<id>")]
pub async fn category(db: ConnectionManager<'_>, id: i32) -> Result<status::Accepted<Json<Vec<Post>>>, status::BadRequest<String>> {
    match repository::post::get_post_by_category(db.0, id).await {
        Ok(x) => Ok(status::Accepted(Some(Json(x)))),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}

/// # Deletar uma Post por ID
#[openapi(tag = "Post")]
#[delete("/post/<id>")]
pub async fn delete(db: ConnectionManager<'_>, id: i32) -> Result<status::NoContent, status::BadRequest<String>> {
    match repository::post::delete_post_by_id(db.0, id).await {
        Ok(_x) => Ok(status::NoContent),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}


/// # Insere uma nova Post
#[openapi(tag = "Post")]
#[post("/post", format = "multipart/form-data", data = "<data>")]
pub async fn insert(db: ConnectionManager<'_>, data: FormNewPost) -> Result<status::Created<Json<Post>>, status::BadRequest<String>> {
    let filemanager = S3FileManager::new(None, None, None, None);

    let id = Uuid::new_v4();

    match filemanager.put_file_in_bucket(format!("{}.png", id.to_string()), data.photo.0).await {
        Ok(x) => {
            let new_post = NewPost {
                categoria_id: data.categoria_id,
                conteudo: data.conteudo,
                tipo: data.tipo,
                language:data.language,
                titulo: data.titulo,
                img: Some(x),
            };

            match repository::post::insert_post(db.0, &new_post).await {
                Ok(x) => Ok(status::Created::new("".to_string()).body(Json(x))),
                Err(x) => Err(status::BadRequest(Some(x.to_string())))
            }
        }
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}

/// # Atualiza uma nova Post
#[openapi(tag = "Post")]
#[put("/post", format = "application/json", data = "<task>")]
pub async fn update(db: ConnectionManager<'_>, task: Json<Post>) -> Result<status::Created<Json<Post>>, status::BadRequest<String>> {
    let new_post = NewPost {
        titulo: task.0.titulo,
        img: task.0.img,
        language:task.0.language,
        categoria_id: task.0.categoria_id.unwrap(),
        tipo: task.0.tipo,
        conteudo: task.0.conteudo,
    };

    match repository::post::update_post_by_id(db.0, task.0.id, &new_post).await {
        Ok(x) => Ok(status::Created::new("".to_string()).body(Json(x))),
        Err(x) => Err(status::BadRequest(Some(x.to_string())))
    }
}