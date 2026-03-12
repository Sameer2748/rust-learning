use axum::{Extension, extract::State, Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;
use crate::AppState;
use common::{CreateMonitorRequest, Monitor};
use axum::extract::Path; 
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PausePayload {
    pub paused: bool,
}

pub async fn create_monitor(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(payload): Json<CreateMonitorRequest>,
) -> impl IntoResponse 
{
// ) -> Result<StatusCode, StatusCode> {
    println!("monitor created is {:?}, {:?}", user_id, payload);


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

pub async fn getAllMonitor( State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,) -> impl IntoResponse
{
    // Fix missing comma after user_id and added created_at!, paused!
     let result = sqlx::query_as!(
        Monitor,
        r#"
        SELECT id ,url, user_id, period, last_heartbeat, paused as "paused!", created_at as "created_at!" 
        FROM websites 
        where user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) =>   {
            eprintln!("Error fetching monitors: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch monitors").into_response()
        }
    }
}

pub async fn getMonitorById( State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
     Path(website_id): Path<Uuid>,)-> impl IntoResponse{
 // Fix missing comma after user_id and added created_at!, paused!
     let result = sqlx::query_as!(
        Monitor,
        r#"
        SELECT id ,url, user_id, period, last_heartbeat, paused as "paused!", created_at as "created_at!" 
        FROM websites 
        where id = $1 and  user_id = $2 
        ORDER BY created_at DESC
        "#,
        website_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(monitor)) => (StatusCode::OK, Json(monitor)).into_response(), // It exists!
        Ok(None) => (StatusCode::NOT_FOUND, "Monitor not found").into_response(), // 404!
        Err(e) => {
            eprintln!("Error fetching monitor: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch monitor").into_response()
        }
    }
}
pub async fn toggleMonitor(
    State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(website_id): Path<Uuid>,
    Json(payload): Json<PausePayload>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
        UPDATE websites 
        SET paused = $1 
        WHERE id = $2 AND user_id = $3
        RETURNING id
        "#,
        payload.paused,
        website_id,
        user_id
    )
    .fetch_optional(&state.db)
    .await;
    match result {
        Ok(Some(_)) => (StatusCode::OK, "Monitor pause status updated").into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Monitor not found").into_response(),
        Err(e) => {
            eprintln!("Error pausing monitor: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update monitor").into_response()
        }
    }
}

pub async fn deleteMonitor( State(state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(website_id): Path<Uuid>,)-> impl IntoResponse{

    let result = sqlx::query!(
        r#"
        DELETE FROM websites WHERE id = $1 AND user_id = $2 RETURNING id
        "#,
        website_id,
        user_id
    ).fetch_optional(&state.db).await;

    match result {
Ok(Some(_)) => (StatusCode::OK, "Monitor deleted successfully").into_response(),        Ok(None) => (StatusCode::NOT_FOUND, "Monitor not found").into_response(),
        Err(e)=>{
        eprintln!("Eror deleting this monitor");
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete monitor").into_response()
        }
    }
}
