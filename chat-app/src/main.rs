use axum::{
    Router,
    extract::{State, ws::{Message, WebSocket, WebSocketUpgrade}},
    response::Response,
    routing::get 
};
use std::net::SocketAddr;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// so this was for incoming data like type of data user will send in the websocket 
#[derive(serde::Deserialize)] 
enum ChatAction {
    Join { room: String, username: String },
    ChatMessage { 
        room: String,    // <--- Added this
        content: String 
    },
}


// now we will create stuct for how we will store the data of chatrooms 
#[derive(serde::Serialize, Debug)]
struct ChatMessage{
    id: String,
    user_id: String,
    username: String,
    content: String
}
#[derive(Debug)]
struct ChatRoom{
    name: String,
    history: Vec<ChatMessage>,
    participants: HashMap<String, String>, 

}
// struct for state of our app 
#[derive(Debug)]
struct AppState {
    rooms: std::collections::HashMap<String, ChatRoom>,
}

#[tokio::main]
pub async fn main(){
    let state = Arc::new(Mutex::new(AppState {
        rooms: HashMap::new()
    }));
    let app = Router::new().route("/ws", get(convert_route_to_ws)).with_state(state);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener  = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}   

pub async fn convert_route_to_ws( State(state) : State<Arc<Mutex<AppState>>>, ws: WebSocketUpgrade)-> Response{
    ws.on_upgrade(|socket| handle_websocket_connection(socket, state))
}

pub async fn handle_websocket_connection(mut ws:WebSocket, state : Arc<Mutex<AppState>>){
    let mut current_chatroom : Option<String> = None;   
    let mut username : Option<String> = None;
    let mut current_user_id : Option<String> = None;
   

    // we wait till we get some msg
    while let Some(msg) = ws.recv().await {
        // this msg we get is of type Result<Message, Error>
        let msg = if let Ok(msg) = msg{
            msg
        } else {
            return
        };

        println!("msg received: '{:?}'", msg);

        if let Message::Text(text) = msg {
            if let Ok(action)  = serde_json::from_str::<ChatAction>(&text){
                match action {
                    ChatAction::Join {room , username: name_from_json } => {
                        
                        let user_id = uuid::Uuid::new_v4().to_string(); 
                        current_chatroom = Some(room.clone());
                        username = Some(name_from_json.clone());
                        current_user_id = Some(user_id.clone()); 
                        {
                            let mut locked_state = state.lock().unwrap();
                            let room_obj = locked_state.rooms.entry(room.clone()).or_insert_with(|| ChatRoom {
                                name: room.clone(),
                                history: Vec::new(),
                                participants: HashMap::new(),
                            });
                            room_obj.participants.insert(user_id.clone(), name_from_json.clone());
                            println!("all state {:?} ", locked_state);
                        }
                        ws.send(Message::Text(format!("You have joined the chat with user id  {}", user_id))).await.unwrap();
                    }
                     ChatAction::ChatMessage { room, content } => {
                        let content_id = uuid::Uuid::new_v4().to_string();
                        let chat_message = ChatMessage {
                            id: content_id,
                            user_id: current_user_id.clone().unwrap(),
                            username: username.clone().unwrap(),
                            content: content.clone(),
                        };
                        let mut locked_state = state.lock().unwrap();
                        let room_obj = locked_state.rooms.get_mut(&room).unwrap();
                        room_obj.history.push(chat_message);
                        println!("all state {:?} ", locked_state);
                    }
                }
            }

        }
    }
}