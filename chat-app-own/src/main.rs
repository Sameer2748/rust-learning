// all the library we need in this file
use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::get,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
// now we define the enums of action what type of action we can get from the user
#[derive(serde::Deserialize)]
enum ChatAction {
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
#[derive(serde::Serialize, Debug, Clone)]
struct ChatMessage {
    id: String,
    name: String,
    userId: String,
    content: String,
}
#[derive(serde::Serialize, Debug, Clone)]
struct User {
    id: String,
    name: String,
}

#[derive(Debug, serde::Serialize, Clone)]
struct ChatRoom {
    id: String,
    ownerId: String,
    name: String,
    history: Vec<ChatMessage>,
    participants: Vec<User>,
    // now then transmitter
    #[serde(skip)]
    tx: broadcast::Sender<ChatMessage>,
}
#[derive(Debug)]
struct AppState {
    rooms: HashMap<String, ChatRoom>,
}

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

pub async fn convert_into_ws(
    State(state): State<Arc<Mutex<AppState>>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(|socket| handleWebsocket(socket, state))
}

pub async fn handleWebsocket(mut socket: WebSocket, state: Arc<Mutex<AppState>>) {
    let mut current_username: Option<String> = None;
    let mut current_userId: Option<String> = None;
    let mut room_rx: Option<broadcast::Receiver<ChatMessage>> = None;

    loop {
        // this tokio is like a process but the main thing is it let us wait for multiple workers like in this
        tokio::select! {
            // nopw in this we have two type of channel or type first one is  for checking the incominng mssage and sending to transmitter for every riikld
            incoming_msg = socket.recv() =>  {
                // we got a option type msg
                let msg = match incoming_msg {
                    Some(Ok(msg))=> msg,
                    _ => break,
                };

                // now chekc if it is correct. type of msg
                if let Message::Text(text) = msg {
                    if let Ok(action) = serde_json::from_str::<ChatAction>(&text){
                        // now we match the type of chat msg we got from the enums
                        match action {
                        ChatAction::CreateRoom {ownerId: ownerId_cl, name, username} => {
                            let roomId = uuid::Uuid::new_v4().to_string();
                            let (tx, rx)  = broadcast::channel(100);
                            let chatRoom = ChatRoom {
                                id: roomId.clone(),
                                ownerId: ownerId_cl.clone(),
                                name: name.clone(),
                                history: Vec::new(),
                                participants: Vec::new(),
                                tx
                            };
                            // now for noe we are suing this locked state but we are not unlocking it to use it
                            // let mut locked_state = state.lock().unwrap();
                            // locked_state.rooms.insert(room_id.clone(), chatmessage);
                            {
                                let mut locked_state = state.lock().unwrap();
                                locked_state.rooms.insert(roomId.clone(), chatRoom);
                            }

                            // and now we send the msg to the user that this is your room id that you jsut created
                            let _ = socket.send(Message::Text(format!("successfully created room wiuth this id {}", roomId.clone() ))).await;
                        }
                        ChatAction::Join {name, userId, roomId } => {
                            current_username = Some(name.clone());
                            current_userId = Some(userId.clone());
                            let user = User{
                                id: userId.clone(),
                                name: name.clone()
                            };
                            let mut not_found = false;

                            {
                                let mut locked_state = state.lock().unwrap();
                                if let Some(room_obj) = locked_state.rooms.get_mut(&roomId) {
                                    room_obj.participants.push(user);
                                    // now we subscribe to thentransimitter ofthis room
                                    room_rx = Some(room_obj.tx.subscribe());
                                }else{
                                    not_found = true;
                                }
                            }
                            if not_found {
                                let _ =  socket.send(Message::Text(format!("room with id {} not found", roomId.clone()))).await;
                            }else {
                                let _ =  socket.send(Message::Text(format!("Joined the room ,{}", roomId.clone()))).await;
                            }
                        }
                        ChatAction::SendMessage {roomId, content} => {
                            let contentId = uuid::Uuid::new_v4().to_string();
                            let message = ChatMessage{
                                id: contentId.clone(),
                                // as_ref(): "Let me look at what's inside the Option without taking it out."
                                // unwrap(): "I know there's a value here, just give it to me."
                                // clone(): "Make a copy for the new message."
                                userId: current_userId.as_ref().unwrap().clone(),
                                name: current_username.as_ref().unwrap().clone(),
                                content: content.clone()
                            };
                            {
                                let mut locked_state = state.lock().unwrap();
                                if let Some(room_obj) = locked_state.rooms.get_mut(&roomId) {
                                    room_obj.history.push(message.clone());
                                    let _ = room_obj.tx.send(message);
                                }
                            }
                            let _=  socket.send(Message::Text(format!("msg send successfully in this room id ,{}", roomId.clone()))).await;
                        }
                        ChatAction::DeleteMessage {roomId , contentId} => {
                            
                            let mut deleted = false;
                            {
                                let mut locked_state = state.lock().unwrap();
                                if let Some(room_obj) = locked_state.rooms.get_mut(&roomId) {
                                    let initial_len = room_obj.history.len();
                                    room_obj.history.retain(|msg| msg.id != contentId.clone());
                                    if room_obj.history.len() < initial_len {
                                        deleted = true;
                                    }
                                }
                            }

                            if deleted {
                                let _=  socket.send(Message::Text(format!("msg deleted with id ,{}", contentId.clone()))).await;
                            }else {
                                let _=  socket.send(Message::Text(format!("msg not found with id ,{}", contentId.clone()))).await;
                            }
                        }
                        ChatAction::LeaveRoom {roomId} => {
                            {
                                let mut locked_state = state.lock().unwrap();
                                if let Some(room_obj) = locked_state.rooms.get_mut(&roomId) {
                                    room_obj.participants.retain(|user| user.id  != current_userId.as_ref().unwrap().clone());
                                }
                            }
                             let _=  socket.send(Message::Text(format!("exited room with id ,{}", roomId.clone()))).await;
                        }
                        }
                    }
                }
            }

            broadcast_msg = async {
                match &mut room_rx {
                    Some( rx) => rx.recv().await.ok(),
                    None => std::future::pending().await
                }
            }=>{
                if let Some(msg)  = broadcast_msg {
                    let json = serde_json::to_string(&msg).unwrap();
                    let _ = socket.send(Message::Text(json)).await;
                }

            }
        }
    }
}

// {"CreateRoom": {"name": "testing 1", "ownerId": "1", "username": "manmohan"}}
// {"Join": {"name":"sameer", "userId":"3", "roomId": "6f93b7f3-b707-49d3-94ad-cddce07d24cf"}}
// {"SendMessage": { "roomId": "6f93b7f3-b707-49d3-94ad-cddce07d24cf", "content":"hey bros ameer his side"}}
// {"SendMessage": { "roomId": "6f93b7f3-b707-49d3-94ad-cddce07d24cf", "content":"hey bro"}}
// {"DeleteMessage": { "roomId": "f067d296-9950-4094-a8d8-b361f284e796", "contentId":"7ac3f2f6-3107-4bd1-a864-1832eae1852d"}}
