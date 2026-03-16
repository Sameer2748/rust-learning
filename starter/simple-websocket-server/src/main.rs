use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};
use std::net::SocketAddr;

// this make this fn main and make it async
#[tokio::main]
pub async fn main() {
    let app = Router::new().route("/ws", get(convert_http_to_ws));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listner = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listner, app).await.unwrap();
}

// now this function expect this websocketudgrace then axum auto detect the same struct from the request if nto then give error
// and this on_upgrade it convert our normal http to a websocket server
async fn convert_http_to_ws(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_my_socket)
}
// now in this what er have to do is run a whil looop till we find the culprit []
async fn handle_my_socket(mut ws: WebSocket) {
    // this wait for the messagee from other side
    while let Some(msg) = ws.recv().await {
        // now we check first if we got this msg correct or not
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        // let realmsg = Message::Text().await;

        if let Message::Text(text) = msg {
            let trimmed = text.trim(); // 1. Remove hidden newlines from terminal
            println!("msg received: '{}'", trimmed);

            if trimmed.to_lowercase() == "ping" {
                // 2. Correct comparison
                if ws.send(Message::Text("Pong".to_string())).await.is_err() {
                    return;
                }
                println!("Pong sent");
            }
        }
    }
}
