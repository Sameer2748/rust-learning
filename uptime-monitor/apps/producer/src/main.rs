use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use chrono::Utc;
use common::Monitor;
use redis::AsyncCommands; // Needed to use `.xadd()`

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Starting Uptime Producer...");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    println!("Producer connected to DB successfully!");

    // Connect to Redis
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = redis::Client::open(redis_url)?;
    let mut redis_con = redis_client.get_async_connection().await?;
    println!("Producer connected to Redis successfully!");
    
    loop {
        println!("Checking for next monitors...");
        
          let result = sqlx::query_as!(
                Monitor,
                r#"
                SELECT id ,url, user_id, period, last_heartbeat, paused as "paused!", created_at as "created_at!" 
                FROM websites 
                ORDER BY created_at DESC
                "#,
            )
            .fetch_all(&pool)
            .await;

            match result {
                Ok(result) => {
                    println!("Found {} total monitors", result.len());
                    let now = Utc::now();
                    for monitor in result {
                        if monitor.paused {
                            continue;
                        }

                        let is_due = match monitor.last_heartbeat {
                            None => true,
                            Some(last_check) => {
                                let diff_sec = (now - last_check).num_seconds();
                                diff_sec >= (monitor.period as i64)
                            }
                        };

                        if is_due {
                            println!("Queueing job for monitor: {} - {}", monitor.id, monitor.url);

                            // Push to Redis Stream!
                            let _: () = redis_con.xadd("uptime_jobs", "*", &[
                                ("monitor_id", monitor.id.to_string()), 
                                ("url", monitor.url.clone())
                            ]).await?;

                            // Update last_heartbeat immediately so it doesn't get queued again next loop!
                            sqlx::query!(
                                "UPDATE websites SET last_heartbeat = $1 WHERE id = $2",
                                now,
                                monitor.id
                            ).execute(&pool).await?;
                        }
                    }
                },
                Err(e) =>   {
                    eprintln!("Error fetching monitors: {:?}", e);
                }
            }
           
            tokio::time::sleep(Duration::from_secs(120)).await;
    }
}
