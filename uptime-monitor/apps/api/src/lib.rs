pub mod handlers;
pub mod middleware;
pub mod jwt;

use axum::middleware::from_fn;
use axum::{routing::{get, post, delete, put}, Router};
use sqlx::PgPool;
use std::net::SocketAddr;
use serde::Deserialize;

use crate::middleware::checker::auth_middleware;
use crate::handlers::{auth, monitor};


#[derive(Deserialize)]
pub struct PausePayload {
    pub paused: bool, 
}

//state so we can use the pool
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn run_server(pool: PgPool){
   let state = AppState {db: pool};
   let port = 3002;

    let protected_routes = Router::new()
     .route("/monitor", post(monitor::create_monitor))
     .route("/monitor", get(monitor::getAllMonitor))
        .route("/monitor/:website_id", get(monitor::getMonitorById))
        .route("/monitor/:website_id", delete(monitor::deleteMonitor))
        .route("/monitor/:website_id", put(monitor::toggleMonitor))
    .layer(from_fn(auth_middleware)).with_state(state.clone()); // layer help us add the middleware trait in this router and in that we ahve from_fn which take our middleware funtion 

   
   let app = Router::new()
   .route("/health", get(health_check))
   .route("/signup", post(auth::signup)) // <--- Add this
    .route("/signin", post(auth::signin))  
    .merge(protected_routes)
   .with_state(state);


   let addr = SocketAddr::from(([127,0,0,1], port));
   println!("backend running on port : {:?}" , port);

   let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
   axum::serve(listner, app).await.unwrap();

}

pub async fn health_check ()-> &'static str {
    "Status: OK"
}


