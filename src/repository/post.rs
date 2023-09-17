use diesel::{ExpressionMethods,QueryDsl};
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::Post;
use crate::models::new_models::NewPost;
use crate::models::PgAsyncConnection;
use crate::schema::post;

pub async  fn insert_post(
    conn: &PgAsyncConnection,
    new_post: &NewPost,
) -> AsyncResult<Post> {
    diesel::insert_into(post::table)
        .values(new_post).returning(post::all_columns)
        .get_result_async(conn).await
}

pub async  fn get_all_posts(
    conn: &PgAsyncConnection,
) -> AsyncResult<Vec<Post>> {
    post::table.load_async(conn).await
}

pub async  fn delete_post_by_id(
    conn: &PgAsyncConnection,
    post_id: i32,
) -> AsyncResult<usize> {
    diesel::delete(post::table.filter(post::id.eq(post_id))).execute_async(conn).await
}

pub async  fn get_post_by_id(
    conn: &PgAsyncConnection,
    post_id: i32,
) -> AsyncResult<Post> {
    post::table.filter(post::id.eq(post_id)).first_async(conn).await
}

pub async  fn update_post_by_id(
    conn: &PgAsyncConnection,
    post_id: i32,
    changeset: &NewPost,
) -> AsyncResult<Post> {
  diesel::update(post::table.filter(post::id.eq(post_id)))
        .set(changeset).returning(post::all_columns)
        .get_result_async(conn).await
}



pub async  fn increment_total_views(
    conn: &PgAsyncConnection,
    post_id: i32,
) -> AsyncResult<Post> {
  match  diesel::sql_query(format!(
        "UPDATE post SET total_views = total_views + 1 WHERE id = {}",
        post_id
    ))
        .execute_async(conn).await{
      Ok(_x) => Ok(post::table.find(post_id).first_async(conn).await.unwrap()),
      Err(e) => Err(e)
  }

    
   
   
}