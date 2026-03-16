use serde::{Deserialize, Serialize};


// now we define the enums of action what type of action we can get from the user
#[derive(Deserialize)]
pub enum ChatAction {
    CreateRoom {
        ownerId: String,
        username: String,
        name: String,
    },
    Join {
        name: String,
        userId: String,
        roomId: String,
    },
    SendMessage {
        roomId: String,
        content: String,
    },
    DeleteMessage {
        contentId: String,
        roomId: String,
    },
    LeaveRoom {
        roomId: String,
    },
}

// now we need the stuct first for this chat-app
#[derive(Serialize, Debug, Clone)]
pub struct ChatMessage {
     pub id: String,
     pub name: String,
     pub userId: String,
     pub content: String,
}
#[derive(Serialize, Debug, Clone)]
pub struct User {
     pub id: String,
     pub name: String,
}
