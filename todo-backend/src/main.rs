use axum::{
    routing::{get , post, delete, put},
    Router,
    Json,    
};

mod handlers;

use handlers::{user, todo};

#[tokio::main]
async fn main(){
    let app = Router::new()
    .route("/", get(|| async {"hello from backend"}))
    .route("/signin", post(user::signinuser))
    .route("/signup", post(user::signupuser))
    .route("/getall", get(todo::getalltodo))
    .route("/create", post(todo::createTodo))
    .route("/todo/:id", get(todo::getTodo))
    .route("/todo/:id", delete(todo::deleteTodo))
    .route("/todo/:id", put(todo::updateTodo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}