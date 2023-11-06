use diesel::{ExpressionMethods,QueryDsl};
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::Category;
use crate::models::new_models::NewCategory;
use crate::models::PgAsyncConnection;
use crate::schema::category;

pub async  fn insert_category(conn: &PgAsyncConnection, new : &NewCategory) -> AsyncResult<Category> {
    diesel::insert_into(category::table)
        .values(new)
        .returning(category::all_columns)
        .get_result_async::<Category>(conn).await
}
pub async  fn get_all_categories(conn: &PgAsyncConnection) -> AsyncResult<Vec<Category>> {
    category::table.load_async::<Category>(conn).await
}

pub async  fn delete_category_by_id(conn: &PgAsyncConnection, category_id: i32) -> AsyncResult<usize> {
    diesel::delete(category::table.filter(category::id.eq(category_id)))
        .execute_async(conn).await
}

pub async  fn get_category_by_name(conn: &PgAsyncConnection, str: String) -> AsyncResult<Category> {
    category::table.filter(category::name_url.eq(str)).first_async(conn).await
}
pub async  fn get_category_by_id(conn: &PgAsyncConnection, category_id: i32) -> AsyncResult<Category> {
    category::table.filter(category::id.eq(category_id)).first_async(conn).await

}