use axum::{
    Router,
    routing::get,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

mod models;
mod state;
mod handlers;

use crate::handlers::convert_into_ws;
use crate::state::AppState;

#[tokio::main]
pub async fn main() {
    // we need state of room in that we will; store the roioms and their transmitter with user detials and thechat history
    let state = Arc::new(Mutex::new(AppState {
        rooms: HashMap::new(),
    }));

    let app = Router::new()
        .route("/ws", get(convert_into_ws))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on Port {}", addr);
    axum::serve(listener, app).await.unwrap();
}

