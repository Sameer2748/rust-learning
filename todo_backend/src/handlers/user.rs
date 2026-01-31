use axum::extract::{State, Json};
use serde::{Deserialize, Serialize};
use crate::db::user_fns;
use crate::app_state::AppState;
use crate::auth::jwt::{create_token};


#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub name: String,
    pub password: String,
    pub email: String
}

pub async fn signinuser(State(state): State<AppState>, Json(user): Json<UserData>) -> String {
    println!("user is : {:?}", user);
    match user_fns::signin_user(
        &state.pool,
        user.email,
        user.password,
    )
    .await
    {
        Ok(Some(user_id)) =>{
            let token = create_token(user_id).await;
             format!("User signed in successfully with id: {}, token: {}", user_id, token)
            },
        Ok(None) => "Invalid email or password".to_string(),
        Err(e) => format!("Error signing in: {}", e),
    }
}

pub async fn signupuser(State(state): State<AppState>, Json(user): Json<UserData>) -> String {
    println!("user is : {:?}", user);
    match user_fns::signup_user(
        &state.pool,
        user.name,
        user.email,
        user.password,
    )
    .await
    {
        Ok(user_id) =>{
            let token = create_token(user_id).await;
             format!("User created successfully with id: {}, token: {}", user_id, token)
            },
        Err(e) => format!("Error creating user: {}", e),
    }
}