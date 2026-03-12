pub mod auth;
pub mod monitor;


async fn health_check() -> String {
    "OK".to_string()
}
