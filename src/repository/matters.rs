use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::Matters;
use crate::models::new_models::NewMatters;
use crate::models::PgAsyncConnection;
use crate::schema::matters;
use diesel::{ExpressionMethods,QueryDsl};

pub async  fn insert(conn: &PgAsyncConnection, new : &NewMatters) -> AsyncResult<Matters> {
    diesel::insert_into(matters::table)
        .values(new)
        .returning(matters::all_columns)
        .get_result_async::<Matters>(conn).await
}
pub async  fn get_all(conn: &PgAsyncConnection) -> AsyncResult<Vec<Matters>> {
    matters::table.load_async::<Matters>(conn).await
}

pub async  fn delete_by_id(conn: &PgAsyncConnection, category_id: i32) -> AsyncResult<usize> {
    diesel::delete(matters::table.filter(matters::id.eq(category_id)))
        .execute_async(conn).await
}

pub async  fn get_by_button(conn: &PgAsyncConnection, str: String) -> AsyncResult<Matters> {
    matters::table.filter(matters::button.eq(str)).first_async(conn).await
}
pub async  fn get_by_id(conn: &PgAsyncConnection, category_id: i32) -> AsyncResult<Matters> {
    matters::table.filter(matters::id.eq(category_id)).first_async(conn).await

}