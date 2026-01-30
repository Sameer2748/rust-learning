use axum::{
    routing::{get , post, delete, put},
    Router,
    Json,    
};

mod handlers;
mod db;

use handlers::{user, todo};
use db::connection::connect_db;

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

    connect_db().await;
    println!("connected db successfully ");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}