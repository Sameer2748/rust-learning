use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::AppState; 
use crate::jwt::jwt::create_token;
use common::{SignInData, SignUpData, AuthResponse, UserResponse};
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST}; 


pub async fn signup(  
    State(state): State<AppState>,
    Json(user): Json<SignUpData>
)-> impl IntoResponse{
    let hashed_password = hash(user.password, DEFAULT_COST).expect("Failed to hash password");
    let user = sqlx::query!(
         "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING id, email, name",
        user.email,
        hashed_password,
        user.name
    ).fetch_one(&state.db).await;

   

    match user {
        Ok(data)=> {
            let user_resp = UserResponse {
                 id: data.id,
                 email: data.email,
                 name: data.name,
            };
             let response = get_details(user_resp).await;
            (StatusCode::CREATED, Json(response)).into_response()
        },
        Err(_)=> (StatusCode::CONFLICT, "user already exists").into_response()
    }
}
pub async fn signin(
     State(state): State<AppState>,
    Json(user): Json<SignInData>
)-> impl IntoResponse{
    let result = sqlx::query!(
        "SELECT id, password_hash, name, email FROM users WHERE email = $1",
        user.email
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(user_db))=>{
            if verify(user.password, &user_db.password_hash).expect("Failed to verify") {
                let user_resp = UserResponse {
                 id: user_db.id,
                 email: user_db.email,
                 name: user_db.name,
            };
                 let response = get_details(user_resp).await;
                (StatusCode::OK, Json(response)).into_response()
            }else {
                (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
            }
        }
        Ok(None)=> (StatusCode::NOT_FOUND, "User not found").into_response(),
        Err(_)=> (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
    }

}

async fn get_details(data:UserResponse)-> AuthResponse{
    let token = create_token(data.id).await;
    let user = UserResponse {id:data.id, email: data.email, name: data.name};
    AuthResponse {token , user}
}