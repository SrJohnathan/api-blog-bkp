use crate::models::models::Ads;
use crate::models::PgAsyncConnection;
use crate::schema::ads;
use diesel::{ExpressionMethods, QueryDsl};
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::new_models::NewAds;


pub async  fn insert_ads(conn: &PgAsyncConnection, new : &NewAds) -> AsyncResult<Ads> {
    diesel::insert_into(ads::table)
        .values(new)
        .returning(ads::all_columns)
        .get_result_async::<Ads>(conn).await
}
pub async  fn get_all_categories(conn: &PgAsyncConnection) -> AsyncResult<Vec<Ads>> {
    ads::table.load_async::<Ads>(conn).await
}

pub async  fn delete_ads_by_id(conn: &PgAsyncConnection, ads_id: i32) -> AsyncResult<usize> {
    diesel::delete(ads::table.filter(ads::id.eq(ads_id)))
        .execute_async(conn).await
}

pub async  fn get_ads_by_id(conn: &PgAsyncConnection, ads_id: i32) -> AsyncResult<Ads> {
    ads::table.filter(ads::id.eq(ads_id)).first_async(conn).await

}