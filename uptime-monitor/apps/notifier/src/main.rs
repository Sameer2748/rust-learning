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
                    "🔔 Sending email alert for {}: {} status changed from {} to {}!",
                    user_data.name, url, old_status, new_status
                );
                
                let api_key = env::var("BREVO_API_KEY").expect("BREVO_API_KEY must be set");
                let res = send_email(&api_key, &user_data.email, &user_data.name, &url, &new_status).await;

                match res {
                    Ok(_) => println!("✅ Email sent successfully to {}", user_data.email),
                    Err(e) => eprintln!("❌ Failed to send email: {:?}", e),
                }
                
                // 4. Acknowledge
                let _: () = redis_con.xack(stream_name, group_name, &[&message.id]).await?;
            }
        }
    }
}

async fn send_email(
    api_key: &str,
    to_email: &str,
    to_name: &str,
    url: &str,
    status: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let subject = if status == "Up" {
        format!("🟢 Recovery: {} is UP", url)
    } else {
        format!("🚨 Alert: {} is DOWN", url)
    };

    let color = if status == "Up" { "#10b981" } else { "#ef4444" };

    let html_content = format!(
        r#"
        <div style="font-family: sans-serif; padding: 20px; border: 1px solid #e2e8f0; border-radius: 8px; max-width: 600px;">
            <h2 style="color: {color};">Website Status Change</h2>
            <p>Your monitor for <strong>{url}</strong> has changed status to <strong>{status}</strong>.</p>
            <div style="background: #f1f5f9; padding: 15px; border-radius: 4px; margin: 20px 0;">
                <strong>Status:</strong> {status}<br/>
                <strong>Time:</strong> {time}
            </div>
            <p>Check your dashboard for more details.</p>
            <a href="http://localhost:3000/dashboard" style="display: inline-block; background: #000; color: #fff; padding: 10px 20px; text-decoration: none; border-radius: 6px;">View Dashboard</a>
        </div>
        "#,
        color = color,
        url = url,
        status = status,
        time = chrono::Utc::now().to_rfc2822()
    );

    let payload = serde_json::json!({
        "sender": { "name": "NightWatch Alerts", "email": "mrao27488@gmail.com" },
        "to": [{ "email": to_email, "name": to_name }],
        "subject": subject,
        "htmlContent": html_content
    });

    let response = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let err_text = response.text().await?;
        Err(format!("Brevo API error: {}", err_text).into())
    }
}
