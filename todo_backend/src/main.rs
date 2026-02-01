use axum::{
    routing::{get, post, delete, put},
    Router,
    middleware
};

mod handlers;
mod db;
mod app_state;
mod auth;
mod checker;


use handlers::{user, todo};
use app_state::AppState;
use db::connection::connect_db;
use checker::mid::auth_middleware;

#[tokio::main]
async fn main(){
    let pool = connect_db().await;
    let state = AppState { pool };
    println!("connected db successfully ");
    
    let protected_routes = Router::new()
    .route("/getall", get(todo::getalltodo))
    .route("/create", post(todo::create_Todo))
    .route("/todo/:id", get(todo::getTodo))
    .route("/todo/:id", delete(todo::deleteTodo))
    .route("/todo/:id", put(todo::updateTodo))
    .layer(middleware::from_fn(auth_middleware)).with_state(state.clone()); // layer help us add the middleware trait in this router and in that we ahve from_fn which take our middleware funtion 

    let app = Router::new()
        .route("/", get(|| async {"hello from backend"}))
        .route("/signin", post(user::signinuser))
        .route("/signup", post(user::signupuser))
        .merge(protected_routes)
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}