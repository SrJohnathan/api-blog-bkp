use crate::models::models::Ads;
use crate::models::PgAsyncConnection;
use crate::schema::ads;
use diesel::{ExpressionMethods, QueryDsl};

use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::new_models::{NewAds, ResAds};


pub async fn insert_ads(conn: &PgAsyncConnection, new: &NewAds) -> AsyncResult<Ads> {
    diesel::insert_into(ads::table)
        .values(new)
        .returning(ads::all_columns)
        .get_result_async::<Ads>(conn).await
}

pub async fn get_all_categories(conn: &PgAsyncConnection) -> AsyncResult<Vec<Ads>> {
    ads::table.load_async::<Ads>(conn).await
}

pub async fn delete_ads_by_id(conn: &PgAsyncConnection, ads_id: i32) -> AsyncResult<usize> {
    diesel::delete(ads::table.filter(ads::id.eq(ads_id)))
        .execute_async(conn).await
}

pub async fn get_ads_by_id(conn: &PgAsyncConnection, ads_id: i32) -> Result<ResAds, String> {


    match ads::table.filter(ads::id.eq(ads_id)).first_async::<Ads>(conn).await {
        Ok(x) => {


            let mut file = Vec::new();
            let n = &x.images.into_iter();
            let mut iter = n.clone().into_iter();
            while let Some(x2) = iter.next() {
                let i = x2.unwrap();
                let str = match crate::repository::files::get(conn, i).await {
                    Ok(x) => { x.name }
                    Err(_e) => { "".to_string() }
                };
                file.push(str)
            }


            let ads = ResAds {
                id: x.id,
                description: x.description,
                images: file,
                time: x.time,
                url: x.url.iter().map(|x1| { x1.clone().unwrap() }).collect(),
                active: x.active,
                alt: x.alt,
            };
            Ok(ads)
        }
        Err(e) => {
            println!("{}",e.to_string());
            Err(e.to_string())}
    }
}