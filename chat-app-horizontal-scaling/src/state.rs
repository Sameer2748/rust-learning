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
    #[serde(skip)]
    pub tx: broadcast::Sender<ChatMessage>,
}
#[derive(Debug)]
pub struct AppState {
    pub rooms: HashMap<String, ChatRoom>,
}