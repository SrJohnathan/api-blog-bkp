use diesel::{ExpressionMethods, QueryDsl, sql_query};


use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::{Category};
use diesel::prelude::*;
use futures::future::join_all;
use futures::StreamExt;
use tokio::join;

use crate::models::new_models::{CategoryWithTotalPosts, NewCategory};
use crate::models::PgAsyncConnection;
use crate::schema::{category, post};


pub async fn insert_category(conn: &PgAsyncConnection, new: &NewCategory) -> AsyncResult<Category> {
    diesel::insert_into(category::table)
        .values(new)
        .returning(category::all_columns)
        .get_result_async::<Category>(conn).await
}

pub async fn get_all_categories(conn: &PgAsyncConnection) -> AsyncResult<Vec<Category>> {
    category::table.load_async::<Category>(conn).await
}

pub async fn delete_category_by_id(conn: &PgAsyncConnection, category_id: i32) -> AsyncResult<usize> {
    diesel::delete(category::table.filter(category::id.eq(category_id)))
        .execute_async(conn).await
}

pub async fn get_category_by_name(conn: &PgAsyncConnection, str: String) -> AsyncResult<Category> {
    category::table.filter(category::name_url.eq(str)).first_async(conn).await
}

pub async fn get_category_by_id(conn: &PgAsyncConnection, category_id: i32) -> AsyncResult<Category> {
    category::table.filter(category::id.eq(category_id)).first_async(conn).await
}

type CategoryWithTotalPostsTuple = (i32, String, String, String, String, String, bool, i64);

pub async fn get_categories_with_total_posts(conn: &PgAsyncConnection) -> Result<Vec<CategoryWithTotalPosts>, String> {
    match  category::table.load_async::<Category>(conn).await   {
        Ok(ff) => {

            let mut vc = Vec::new();

            for x in ff {

                let cot = post::table.filter(post::dsl::categoria_id.eq(x.id)).count().get_result_async(conn).await.unwrap();


                let c: CategoryWithTotalPosts = CategoryWithTotalPosts {
                    id: x.id,
                    name_url: x.name_url.clone(),
                    name_pt: x.name_pt.clone(),
                    name_en: x.name_en.clone(),
                    name_es: x.name_es.clone(),
                    name_fr: x.name_fr.clone(),
                    active: x.active,
                    total_posts: cot,
                };

                vc.push( c )

            }



           Ok( vc)

        }
        Err(x) => Err(x.to_string())
    }


}

// load_async       tokio-diesel = {git = "https://github.com/SrJohnathan/tokio-diesel"  }



