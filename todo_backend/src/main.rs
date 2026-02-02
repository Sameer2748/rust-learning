
use todo_backend::create_router; 

#[tokio::main]
async fn main() {
    let app = create_router().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
