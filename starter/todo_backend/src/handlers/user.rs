use crate::app_state::AppState;
use crate::auth::jwt::create_token;
use crate::db::user_fns;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpData {
    pub name: String,
    pub password: String,
    pub email: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SignInData {
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i32,
}

pub async fn signinuser(
    State(state): State<AppState>,
    Json(user): Json<SignInData>,
) -> impl IntoResponse {
    match user_fns::signin_user(&state.pool, user.email, user.password).await {
        Ok(Some(user_id)) => {
            let token = create_token(user_id).await;
            let response = AuthResponse { token, user_id };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Ok(None) => (
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn signupuser(
    State(state): State<AppState>,
    Json(user): Json<SignUpData>,
) -> impl IntoResponse {

   
    match user_fns::signup_user(&state.pool, user.name, user.email, user.password).await {
        Ok(user_id) => {
            let token = create_token(user_id).await;
            let response = AuthResponse { token, user_id };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
