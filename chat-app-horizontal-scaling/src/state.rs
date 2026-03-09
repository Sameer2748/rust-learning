use crate::models::{ChatMessage, User};
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug, serde::Serialize, Clone)]
pub struct ChatRoom {
    pub id: String,
    pub ownerId: String,
    pub name: String,
    pub history: Vec<ChatMessage>,
    pub participants: Vec<User>,
    // now then transmitter
    // okk now we dont need local transmitter redis will handle it 
    // #[serde(skip)]
    // pub tx: broadcast::Sender<ChatMessage>,
}

use deadpool_redis::Pool;
#[derive(Debug)]
pub struct AppState{
    pub redis_pool : Pool
}