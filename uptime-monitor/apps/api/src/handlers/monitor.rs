use axum::{Extension, extract::State, Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use crate::AppState;
use common::{CreateMonitorRequest, Monitor};

pub async fn create_monitor(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>, // <--- The Middleware puts this here
    Json(payload): Json<CreateMonitorRequest>,
) -> impl IntoResponse {
// ) -> Result<StatusCode, StatusCode> {
    println!("monitor created is {:?}, {:?}", user_id, payload);
    // Now you can insert into SQL:
    // INSERT INTO websites (user_id, url, period ...) VALUES ($1, $2, $3 ...)

   let result = sqlx::query_as!(
        Monitor,
        r#"
        INSERT INTO websites (user_id, url, period) 
        VALUES ($1, $2, $3) 
        RETURNING id, url, user_id, period, last_heartbeat, paused as "paused!", created_at as "created_at!"
        "#,
        user_id,
        payload.url,
        payload.period
    )
    .fetch_one(&state.db)
    .await;
    // 3. Handle the result
    match result {
        Ok(monitor) => {
            println!("Monitor created: {:?}", monitor);
            (StatusCode::CREATED, Json(monitor)).into_response()
        }
        Err(e) => {
            eprintln!("Error creating monitor: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create monitor").into_response()
        }
    }

}


