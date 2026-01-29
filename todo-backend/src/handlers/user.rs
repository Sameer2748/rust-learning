use axum::{
    routing::{get , post},
    Router,
    Json,
    
};
use axum::extract::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    name: String,
    password: String,
    mobile: i64
}

pub async  fn signinuser(Json(user): Json<UserData>){
    println!("user is : {:?}", user );
}
pub async   fn signupuser(Json(user): Json<UserData>){
    println!("user is : {:?}", user );
}