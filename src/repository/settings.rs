use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::Settings;
use crate::models::new_models::NewSettings;
use crate::models::PgAsyncConnection;
use crate::schema::settings;
use diesel::{ExpressionMethods,QueryDsl};

pub async  fn insert(conn: &PgAsyncConnection, new : &NewSettings) -> AsyncResult<Settings> {
    diesel::insert_into(settings::table)
        .values(new)
        .returning(settings::all_columns)
        .get_result_async::<Settings>(conn).await
}
pub async  fn get_all(conn: &PgAsyncConnection) -> AsyncResult<Vec<Settings>> {
    settings::table.load_async::<Settings>(conn).await
}

pub async  fn delete_by_id(conn: &PgAsyncConnection, settings_id: i32) -> AsyncResult<usize> {
    diesel::delete(settings::table.filter(settings::id.eq(settings_id)))
        .execute_async(conn).await
}

pub async  fn get_by_name(conn: &PgAsyncConnection, str: String) -> AsyncResult<Settings> {
    settings::table.filter(settings::name.eq(str)).first_async(conn).await
}
pub async  fn get_by_id(conn: &PgAsyncConnection, settings_id: i32) -> AsyncResult<Settings> {
    settings::table.filter(settings::id.eq(settings_id)).first_async(conn).await

}


pub async fn updade(
    conn: &PgAsyncConnection,
    post_id: i32,
    new : &NewSettings
) -> AsyncResult<Settings> {
    match diesel::update(settings::table.filter(settings::id.eq(post_id)))
        .set(new)
        .returning(settings::all_columns)
        .get_result_async(conn)
        .await {
        Ok(_x) => Ok(_x),
        Err(e) => Err(e)
    }
}