pub mod models;
pub mod new_models;




use std::ops::Deref;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type  PgAsyncConnection =  Pool<ConnectionManager<PgConnection>>;
pub struct   PoolPgAsyncConnection( pub PooledConnection<ConnectionManager<PgConnection>>);


impl Deref for PoolPgAsyncConnection  {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub async  fn connection(str: String) -> Result<PgAsyncConnection,String> {
    let db_url = std::env::var(str).unwrap();
    println!("{}",db_url);
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(manager).unwrap();
    // embed_migrations!();
    Ok(pool)
}