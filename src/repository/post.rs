
use diesel::{ExpressionMethods, QueryDsl};
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::{GetCategory, Post,};
use crate::models::new_models::NewPost;
use crate::models::PgAsyncConnection;
use crate::schema::post;




macro_rules! get_all_category_asc_desc {
    ($category:expr, $table:expr, $id:expr) => {
        {
            let category_parsed = $category.parse::<i32>();
            match category_parsed.is_ok() {
                true => {
                    let category_id = category_parsed.unwrap();
                    let g = $table.filter(categoria_id.eq(category_id)).order($id);
                    GetCategory::ID(g)
                }
                false => {
                    let g = $table.order($id);
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


pub async fn get_last_n_posts(
    conn: &PgAsyncConnection,
    n: i64,
    ord:String,
    category:String
) -> AsyncResult<Vec<Post>> {
    use crate::schema::post::dsl::*;

    match ord.as_str() {
        "asc" => {
            let querry_category : GetCategory<_,_> = get_all_category_asc_desc!(category,post,id.asc());
              match querry_category {
                GetCategory::ALL(x) => x.limit(n).load_async(conn).await,
                GetCategory::ID(x) => x.limit(n).load_async(conn).await
            }
        }
        "desc" =>{
            let querry_category : GetCategory<_,_> = get_all_category_asc_desc!(category,post,id.desc());
            match querry_category {
                GetCategory::ALL(x) => x.limit(n).load_async(conn).await,
                GetCategory::ID(x) => x.limit(n).load_async(conn).await
            }
        }
        _ => {
            let querry_category : GetCategory<_,_> = get_all_category_asc_desc!(category,post,id.asc());
            match querry_category {
                GetCategory::ALL(x) => x.limit(n).load_async(conn).await,
                GetCategory::ID(x) => x.limit(n).load_async(conn).await
            }
        }
    }
    
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
    match diesel::sql_query(format!(
        "UPDATE post SET total_views = total_views + 1 WHERE id = {}",
        post_id
    ))
        .execute_async(conn).await {
        Ok(_x) => Ok(post::table.find(post_id).first_async(conn).await.unwrap()),
        Err(e) => Err(e)
    }
}


