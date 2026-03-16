use axum::{Router, middleware, routing::{delete, get, post, put}};

use crate::{app_state::AppState, checker::mid::auth_middleware, db::connection::connect_db, handlers::{todo, user}};

pub mod app_state;
pub mod db;
pub mod handlers;
pub mod auth;
pub mod checker;


pub async fn create_router()-> Router{
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

    Router::new()
        .route("/", get(|| async {"hello from backend"}))
        .route("/signin", post(user::signinuser))
        .route("/signup", post(user::signupuser))
        .merge(protected_routes)
        .with_state(state)
}