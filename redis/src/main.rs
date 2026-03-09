use deadpool_redis::{Config, Pool, Runtime};
use redis::AsyncCommands;
use std::collections::HashMap;

use redis::streams::{StreamReadOptions, StreamReadReply};

use redis::{Commands, RedisResult};

fn string_examples() -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _: () = con.set("greeting", "hello world")?;
    let v: String = con.get("greeting")?;
    println!("greeting = {}", v);

    let _: () = con.set("page_views", 0)?;
    let new_count: i32 = con.incr("page_views", 2)?;
    println!("page_views = {}", new_count);

    let json = r#"{"id": 1, "name": "Alice"}"#;
    let _: () = con.set("user:1:json", json)?;

    let _: () = con.hset_multiple("user:1", &[("name", "sameer"), ("age", "40")])?;

    let mut user: HashMap<String, String> = con.hgetall("user:1")?;
    println!("user = {:?}", user);

    user.insert("name".to_string(), "Nancy".to_string());
    println!("user = {:?}", user);

    let user_data: i32 = con.hincr("user:1", "age", 19)?;
    println!("user_data = {:?}", user_data);

    let task1 = r#"{"action": "send_email", "to": "user@example.com"}"#;
    let task2 = r#"{"action": "generate_report", "id": 42}"#;

    let _: () = con.lpush("queue:tasks", task1)?;
    let _: () = con.lpush("queue:tasks", task2)?;

    let len: i64 = con.llen("queue:tasks")?;
    println!("Queue length: {}", len);

    for i in 0..len {
        let processing: String = con.rpop("queue:tasks", Default::default())?;
        println!("Processing task: {}", processing);
    }

    let task1 = r#"{"action": "send_email", "to": "user@example.com"}"#;
    let _: () = con.lpush("queue:tasks", task1)?;

    println!("--- Reliable Test ---");

    let task: String = con.rpoplpush("queue:tasks", "queue:processing")?;
    println!("Step 1: Task moved to processing: {}", task);

    let proc_count: i64 = con.llen("queue:processing")?;
    println!("Step 2: Items in processing list: {}", proc_count);

    println!("Step 3: Processing work...");

    let _: () = con.lrem("queue:processing", 1, &task)?;
    println!("Step 4: Task finished and removed from processing.");

    let final_proc_count: i64 = con.llen("queue:processing")?;
    println!("Final processing count: {}", final_proc_count);

    Ok(())
}

async fn async_stream_worker(pool: Pool, worker_id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let stream_name = "test-1";
    let group_name = "my_workers";
    let consumer_name = format!("worker_{}", worker_id);

    let mut con = pool.get().await?;

    let _: RedisResult<()> = con.xgroup_create(stream_name, group_name, "0").await;

    println!("[{}] Ready and waiting...", consumer_name);

    loop {
        let opts = StreamReadOptions::default()
            .group(group_name, &consumer_name)
            .count(1)
            .block(0);

        let reply: StreamReadReply = con.xread_options(&[stream_name], &[">"], &opts).await?;

        for stream in reply.keys {
            for message in stream.ids {
                println!("[{}] Processing job: {}", consumer_name, message.id);

                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                let _: i32 = con.xack(stream_name, group_name, &[&message.id]).await?;
                println!("[{}] Job finished & Acked", consumer_name);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::from_url("redis://127.0.0.1/");
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    let mut producer_con = pool.get().await?;
    println!("Pushing 100 jobs to backlog...");
    for i in 1..=100 {
        let _: String = producer_con
            .xadd("test-1", "*", &[("job", &i.to_string())])
            .await?;
    }
    println!("Finished pushing history.");

    for i in 1..=20 {
        let worker_pool = pool.clone();
        tokio::spawn(async move {
            if let Err(e) = async_stream_worker(worker_pool, i).await {
                eprintln!("Worker #{} encountered an error: {}", i, e);
            }
        });
    }
    tokio::signal::ctrl_c().await?;
    Ok(())
}
