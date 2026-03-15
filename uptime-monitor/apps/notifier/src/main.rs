use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use redis::AsyncCommands;
use redis::streams::{StreamReadOptions, StreamReadReply};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Starting Uptime Notifier...");

    // 1. Connect to PostgreSQL (To lookup user emails)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Notifier connected to DB successfully!");

    // 2. Connect to Redis
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = redis::Client::open(redis_url)?;
    let mut redis_con = redis_client.get_async_connection().await?;
    println!("Notifier connected to Redis successfully!");

    // 3. Setup Consumer Group for Alerts
    let stream_name = "alerts";
    let group_name = "notifier_group";
    let consumer_name = "notifier_1";

    let _: redis::RedisResult<()> = redis_con.xgroup_create_mkstream(stream_name, group_name, "$").await;

    println!("Notifier group initialized. Waiting for alert jobs...");

    // 4. The Loop
    loop {
        let opts = StreamReadOptions::default()
            .group(group_name, consumer_name)
            .count(1)
            .block(5000);

        let reply: StreamReadReply = redis_con.xread_options(&[stream_name], &[">"], &opts).await?;

        for stream in reply.keys {
            for message in stream.ids {
                println!("Processing Alert Message: {}", message.id);
                
                // 1. Parse data from Redis
                let monitor_id_str: String = redis::from_redis_value(message.map.get("monitor_id").unwrap())?;
                let url: String = redis::from_redis_value(message.map.get("url").unwrap())?;
                let old_status: String = redis::from_redis_value(message.map.get("old_status").unwrap())?;
                let new_status: String = redis::from_redis_value(message.map.get("new_status").unwrap())?;

                // 2. Lookup User associated with this monitor
                let user_data = sqlx::query!(
                    r#"
                    SELECT u.email, u.name 
                    FROM users u
                    JOIN websites w ON w.user_id = u.id
                    WHERE w.id = $1
                    "#,
                    uuid::Uuid::parse_str(&monitor_id_str)?
                )
                .fetch_one(&pool)
                .await?;

                // 3. Send Notification (Simulated)
                println!(
                    "🔔 ALERT for {}: {} status changed from {} to {}! Sending email to {} ({})",
                    user_data.name, url, old_status, new_status, user_data.email, user_data.name
                );
                
                // In a real app, you'd use a crate like `lettre` or an API like SendGrid/resend here.
                
                // 4. Acknowledge
                let _: () = redis_con.xack(stream_name, group_name, &[&message.id]).await?;
            }
        }
    }
}
