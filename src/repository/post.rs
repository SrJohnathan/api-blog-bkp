
use diesel::{ExpressionMethods, QueryDsl};

use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::{Category, GetCategory, Post};
use crate::models::new_models::{Language, NewPost, PostWithCategory, TipoPost};
use crate::models::PgAsyncConnection;
use crate::schema::post;




macro_rules! get_all_category_asc_desc {
    ($category:expr, $table:expr, $id:expr,$categoria_table:expr) => {
        {
            let category_parsed = $category.parse::<i32>();
            match category_parsed.is_ok() {
                true => {
                    let category_id = category_parsed.unwrap();
                    let g = $table.inner_join($categoria_table).filter(categoria_id.eq(category_id)).order($id);
                    GetCategory::ID(g)
                }
                false => {
                    let g = $table.inner_join($categoria_table).order($id);
                    GetCategory::ALL(g)
                }
            }
        }
    };
}


pub async fn insert_post(
    conn: &PgAsyncConnection,
    new_post: &NewPost,
) -> AsyncResult<Post> {
    diesel::insert_into(post::table)
        .values(new_post).returning(post::all_columns)
        .get_result_async(conn).await
}

pub async fn get_all_posts(
    conn: &PgAsyncConnection,
) -> AsyncResult<Vec<Post>> {
    post::table.load_async(conn).await
}

pub async fn get_all_posts_lang(
    conn: &PgAsyncConnection,lang:Language
) -> AsyncResult<Vec<Post>> {
    post::table.filter(post::dsl::language.eq(lang)).load_async(conn).await
}



pub async fn get_last_n_posts(
    conn: &PgAsyncConnection,
    n: i64,
    offset:i64,
    ord:String,
    category:String,
    lang:Language
) -> Result<Vec<PostWithCategory>,String> {
    use crate::schema::post::dsl::*;

 let  res =  match ord.as_str() {
        "asc" => {
            let querry_category : GetCategory<_,_> = get_all_category_asc_desc!(category,post,id.asc(),crate::schema::category::table);
              match querry_category {
                GetCategory::ALL(x) => x.select((
                    crate::schema::post::all_columns,
                    crate::schema::category::all_columns
                )).filter(language.eq(lang)).or_filter(tipo.eq(TipoPost::Texto)).or_filter(tipo.eq(TipoPost::Html))
                    .limit(n).offset(offset).load_async::<(Post,Category)>(conn).await,
                GetCategory::ID(x) => x.select((
                                                    crate::schema::post::all_columns,
                                                    crate::schema::category::all_columns
                )).or_filter(tipo.eq(TipoPost::Texto)).or_filter(tipo.eq(TipoPost::Html))
                    .filter(language.eq(lang)).limit(n).offset(offset).load_async::<(Post,Category)>(conn).await
            }
        }
        "desc" =>{
            let querry_category : GetCategory<_,_> = get_all_category_asc_desc!(category,post,id.desc(),crate::schema::category::table);
            match querry_category {
                GetCategory::ALL(x) => x.select((
                    crate::schema::post::all_columns,
                    crate::schema::category::all_columns
                )).or_filter(tipo.eq(TipoPost::Texto)).or_filter(tipo.eq(TipoPost::Html))
                    .filter(language.eq(lang)).limit(n).offset(offset).load_async::<(Post,Category)>(conn).await,
                GetCategory::ID(x) => x.select((
                    crate::schema::post::all_columns,
                    crate::schema::category::all_columns
                )).or_filter(tipo.eq(TipoPost::Texto)).or_filter(tipo.eq(TipoPost::Html))
                    .filter(language.eq(lang)).limit(n).offset(offset).load_async::<(Post,Category)>(conn).await
            }
        }
        _ => {
            let querry_category : GetCategory<_,_> = get_all_category_asc_desc!(category,post,id.asc(),crate::schema::category::table);
            match querry_category {
                GetCategory::ALL(x) => x.select((
                    crate::schema::post::all_columns,
                    crate::schema::category::all_columns
                )).or_filter(tipo.eq(TipoPost::Texto)).or_filter(tipo.eq(TipoPost::Html)).filter(language.eq(lang)).limit(n).offset(offset).load_async::<(Post,Category)>(conn).await,
                GetCategory::ID(x) => x.select((
                    crate::schema::post::all_columns,
                    crate::schema::category::all_columns
                )).or_filter(tipo.eq(TipoPost::Texto)).or_filter(tipo.eq(TipoPost::Html)).filter(language.eq(lang)).limit(n).offset(offset).load_async::<(Post,Category)>(conn).await
            }
        }
    };



   let r =  res.unwrap();
     let response =   r.iter().map( |  x1| {

       let pos = &x1.0;
       let category = &x1.1;

        PostWithCategory{
            description: pos.description.clone(),
            id: pos.id,
            titulo: pos.titulo.clone(),
            img: pos.img.clone(),
            language: pos.language.clone(),
            categoria_id: pos.categoria_id.clone(),
            total_views: pos.total_views.clone(),
            data_criacao: pos.data_criacao.clone(),
            tipo: pos.tipo.clone(),
            conteudo: pos.conteudo.clone(),
            name_category: category.name_url.clone(),
        }

    });



  Ok(  response.collect() )

}


pub async fn delete_post_by_id(
    conn: &PgAsyncConnection,
    post_id: i32,
) -> AsyncResult<usize> {
    diesel::delete(post::table.filter(post::id.eq(post_id))).execute_async(conn).await
}

pub async fn get_post_by_id(
    conn: &PgAsyncConnection,
    post_id: i32,
) -> AsyncResult<Post> {
    post::table.filter(post::id.eq(post_id)).first_async(conn).await
}


pub async fn get_post_by_category(
    conn: &PgAsyncConnection,
    category_id: i32,
) -> AsyncResult<Vec<Post>> {
    post::table.filter(post::categoria_id.eq(category_id)).load_async(conn).await
}


pub async fn get_post_by_viwes(
    conn: &PgAsyncConnection,
    category:String,
    limit: i64,
    lang:Language
) -> AsyncResult<Vec<Post>> {

     match category.parse::<i32>().is_ok() {
        true => {

            post::table
                .filter(post::categoria_id.eq(category.parse::<i32>().unwrap()))
                .or_filter(post::tipo.eq(TipoPost::Texto)).or_filter(post::tipo.eq(TipoPost::Html))
                .filter(post::dsl::language.eq(lang))
                .order(post::total_views.desc())
                .limit(limit)
                .load_async(conn).await
        }
        false => {


            post::table
                .order(post::total_views.desc())
                .or_filter(post::tipo.eq(TipoPost::Texto)).or_filter(post::tipo.eq(TipoPost::Html))
                .filter(post::dsl::language.eq(lang))
                .limit(limit)
                .load_async(conn).await

        }

    }



}



pub async fn get_post_by_audio(
    conn: &PgAsyncConnection,
    category:String,
    limit: i64,
    lang:Language
) -> AsyncResult<Vec<Post>> {

    match category.parse::<i32>().is_ok() {
        true => {

            post::table
                .filter(post::categoria_id.eq(category.parse::<i32>().unwrap()))
                .or_filter(post::tipo.eq(TipoPost::Audio))
                .filter(post::dsl::language.eq(lang))
                .order(post::data_criacao.desc())
                .limit(limit)
                .load_async(conn).await
        }
        false => {


            post::table
                .order(post::data_criacao.desc())
                .or_filter(post::tipo.eq(TipoPost::Audio))
                .filter(post::dsl::language.eq(lang))
                .limit(limit)
                .load_async(conn).await

        }

    }



}



pub async fn update_post_by_id(
    conn: &PgAsyncConnection,
    post_id: i32,
    changeset: &NewPost,
) -> AsyncResult<Post> {
    diesel::update(post::table.filter(post::id.eq(post_id)))
        .set(changeset).returning(post::all_columns)
        .get_result_async(conn).await
}


pub async fn increment_total_views(
    conn: &PgAsyncConnection,
    post_id: i32,
) -> AsyncResult<Post> {

   match diesel::update(post::table.filter(post::id.eq(post_id)))
        .set(post::total_views.eq(post::total_views + 1))
        .execute_async(conn)
        .await {
        Ok(_x) => Ok(post::table.find(post_id).first_async(conn).await.unwrap()),
        Err(e) => Err(e)
    }
}


