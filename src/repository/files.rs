use diesel::{ExpressionMethods,QueryDsl};
use tokio_diesel::{AsyncResult, AsyncRunQueryDsl};
use crate::models::models::Files;
use crate::models::new_models::NewFiles;
use crate::models::PgAsyncConnection;
use crate::schema::files;


pub async  fn insert(conn: &PgAsyncConnection, new : String) -> AsyncResult<Files> {
    diesel::insert_into(files::table)
        .values(NewFiles{name:new})
        .returning(files::all_columns)
        .get_result_async::<Files>(conn).await
}

pub async  fn all(conn: &PgAsyncConnection) -> AsyncResult<Vec<Files>> {
    files::table.load_async(conn).await
}

pub async fn delete(conn: &PgAsyncConnection, idd : i32) -> Result<Files, String> {
   match  files::table.filter(files::dsl::id.eq(idd)).first_async::<Files>(conn).await {
       Ok(_x) => {
         match diesel::delete(files::table.filter(files::dsl::id.eq(idd))).execute_async(conn).await{
             Ok(_xx) => {
                 
                 if _xx > 0 {
                     Ok(_x)
                 } else { 
                     Err("Arquivo não encontrado".to_string())
                 }
                 
             }
             Err(e) => Err(e.to_string())
         }

           
       }
       Err(e) => Err(e.to_string())
   }
}

pub async fn get(conn: &PgAsyncConnection, idd : i32) -> AsyncResult<Files> {
    files::table.filter(files::dsl::id.eq(idd)).first_async(conn).await
}