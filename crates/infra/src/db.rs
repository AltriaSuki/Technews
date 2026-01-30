// Infrastructure for Database
use sqlx::{Pool, Sqlite};

pub type DbPool = Pool<Sqlite>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    // TODO: Initialize database connection
    unimplemented!()
}
