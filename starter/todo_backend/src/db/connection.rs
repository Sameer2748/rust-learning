use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

pub async fn connect_db()-> PgPool{
    dotenv().ok();

    let db_url : String = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("{}", db_url);
    PgPoolOptions::new().max_connections(5).connect(&db_url).await.expect("failed to connect to db ")
}