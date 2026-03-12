mod db;

use dotenvy::dotenv;
use api::run_server;

#[tokio::main]
async fn main()-> anyhow::Result<()>{
    dotenv().ok();

    let pool = db::init_db().await;
    sqlx::migrate!("./migrations").run(&pool).await?;
    run_server(pool).await;

    Ok(())
}


