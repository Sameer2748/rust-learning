use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::{SystemTime, UNIX_EPOCH};
use todo_backend::create_router;
use tokio::net::TcpListener;

// heloper 

#[derive(Debug, Deserialize, Serialize)]
struct ResponseData {
    token: String,
    user_id: i32,
}

async fn spawn_app() -> String {
    let app = create_router().await;
    let listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    format!("http://localhost:{}", port)
}

fn random_user() -> Value {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    json!({
        "email": format!("test_{}@example.com", unique_id),
        "name": format!("User_{}", unique_id),
        "password": "password123"
    })
}

#[tokio::test]
async fn test_signup_success() {
    let base_url = spawn_app().await;
    let client = Client::new();
    let user_data = random_user();

    let res = client
        .post(format!("{}/signup", base_url))
        .json(&user_data)
        .send()
        .await
        .unwrap();

    let body = res.json::<ResponseData>().await.unwrap();
    // Verify we got a token back
    assert!(!body.token.is_empty());
}

#[tokio::test]
async fn test_signup_missing_email() {
    let base_url = spawn_app().await;
    let client = Client::new();
    let mut user_data = random_user();
    user_data.as_object_mut().unwrap().remove("email");

    let res = client
        .post(format!("{}/signup", base_url))
        .json(&user_data)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

// complete lifecycle test
#[tokio::test]
async fn test_full_crud_lifecycle() {
    let base_url = spawn_app().await;
    let client = Client::new();

    // signup nad get token
    let user_data = random_user();
    let res = client
        .post(format!("{}/signup", base_url))
        .json(&user_data)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    let auth_data: ResponseData = res.json().await.unwrap();
    let token = auth_data.token;

    let todo_data = json!({
        "title": "Lifecycle Task",
        "description": "Created during full cycle test",
        "tag": "lifecycle"
    });

    let res = client
        .post(format!("{}/create", base_url))
        .header("Authorization", &token)
        .json(&todo_data)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    
    let todo_id: i32 = res.json().await.unwrap();

    // get todo with id 
    let res = client
        .get(format!("{}/todo/{}", base_url, todo_id))
        .header("Authorization", &token)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let fetched_todo: Value = res.json().await.unwrap();
    assert_eq!(fetched_todo["title"], "Lifecycle Task");

    // updating todo
    let update_data = json!({
        "title": "Updated Task",
        "description": "Updated description",
        "tag": "updated"
    });

    let res = client
        .put(format!("{}/todo/{}", base_url, todo_id))
        .header("Authorization", &token)
        .json(&update_data)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    // Verify update response reflects changes
    let updated_todo: Value = res.json().await.unwrap();
    assert_eq!(updated_todo["title"], "Updated Task");

    // getting all todos 
    let res = client
        .get(format!("{}/getall", base_url))
        .header("Authorization", &token)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let all_todos: Vec<Value> = res.json().await.unwrap();
    assert!(!all_todos.is_empty());

    // deleteting todo
    let res = client
        .delete(format!("{}/todo/{}", base_url, todo_id))
        .header("Authorization", &token)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    // verifying the deleted todo
    let res = client
        .get(format!("{}/todo/{}", base_url, todo_id))
        .header("Authorization", &token)
        .send()
        .await
        .unwrap();


    assert!(
        res.status() == StatusCode::INTERNAL_SERVER_ERROR || res.status() == StatusCode::NOT_FOUND
    );
}
