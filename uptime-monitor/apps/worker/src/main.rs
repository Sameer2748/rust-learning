use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::{Duration, Instant};
use redis::AsyncCommands;
use redis::streams::{StreamReadOptions, StreamReadReply};
use common::WebsiteStatus;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    println!("Starting Uptime Worker...");
    let stream_name = "uptime_jobs";
    let group_name = "worker_group";
    let consumer_name = "worker_1";


    // 1. Connect to PostgreSQL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    println!("Worker connected to DB successfully!");

    // 2. Connect to Redis
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = redis::Client::open(redis_url)?;
    let mut redis_con = redis_client.get_async_connection().await?;
    println!("Worker connected to Redis successfully!");

    // 3. Setup Consumer Group
    // Command: XGROUP CREATE uptime_jobs worker_group $ MKSTREAM
    // We use $ to start reading only new messages. MKSTREAM creates the stream if it doesn't exist.
    let _: redis::RedisResult<()> = redis_con.xgroup_create_mkstream("uptime_jobs", "worker_group", "$").await;
    // (We ignore errors here because it usually just means the group already exists)

    println!("Worker group initialized. Waiting for jobs...");

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    // 4. The Loop
    loop {
         let opts = StreamReadOptions::default()
            .group(group_name, &consumer_name)
            .count(1)
            .block(0);

        let reply: StreamReadReply = redis_con.xread_options(&[stream_name], &[">"], &opts).await?;

        for stream in reply.keys {
            for message in stream.ids {
                // 1. Get Data from Redis Message
                let monitor_id_val = message.map.get("monitor_id").unwrap();
                let url_val = message.map.get("url").unwrap();
                
                let monitor_id_str: String = redis::from_redis_value(monitor_id_val)?;
                let url: String = redis::from_redis_value(url_val)?;
                
                let monitor_id = Uuid::parse_str(&monitor_id_str).unwrap();
                println!("Checking: {}", url);

                // 2. Perform the actual Ping
                let start = Instant::now();
                let response = client.get(&url).send().await;
                let duration = start.elapsed().as_millis() as i32;

                let (status, msg) = match response {
                    Ok(resp) if resp.status().is_success() => (WebsiteStatus::Up, None),
                    Ok(resp) => (WebsiteStatus::Down, Some(format!("HTTP {}", resp.status()))),
                    Err(e) => (WebsiteStatus::Down, Some(e.to_string())),
                };

                // 3. Status Change Detection
                // Find the previous status
                let last_tick = sqlx::query!(
                    r#"SELECT status as "status: WebsiteStatus" FROM website_ticks WHERE website_id = $1 ORDER BY created_at DESC LIMIT 1"#,
                    monitor_id
                )
                .fetch_optional(&pool)
                .await?;

                // 3. Status Change Detection
                let mut status_changed = false;
                let mut old_status_str = "None".to_string();

                match last_tick {
                    None => {
                        // First check ever. If it's DOWN, we should alert!
                        if status == WebsiteStatus::Down {
                            status_changed = true;
                            old_status_str = "Initial Check (Down)".to_string();
                        }
                    }
                    Some(tick) => {
                        if tick.status != status {
                            status_changed = true;
                            old_status_str = format!("{:?}", tick.status);
                        }
                    }
                };

                if status_changed {
                    println!("🚀 Alerting! {} is now {:?}", url, status);
                    
                    // Push to alerts stream
                    let _: () = redis_con.xadd(
                        "alerts",
                        "*",
                        &[
                            ("monitor_id", monitor_id.to_string()),
                            ("url", url.clone()),
                            ("old_status", old_status_str),
                            ("new_status", format!("{:?}", status)),
                        ]
                    ).await?;
                }

                // 4. Save the result to PostgreSQL (website_id is the correct column name)
                sqlx::query!(
                    "INSERT INTO website_ticks (id, website_id, status, response_time_ms, message) VALUES ($1, $2, $3, $4, $5)",
                    Uuid::new_v4(),
                    monitor_id,
                    status as WebsiteStatus,
                    duration,
                    msg
                )
                .execute(&pool)
                .await?;

                // 4. ACKNOWLEDGE (XACK)
                let _: () = redis_con.xack(stream_name, group_name, &[&message.id]).await?;
                
                println!("Result: {:?} in {}ms", status, duration);
            }
        }
    }
}
