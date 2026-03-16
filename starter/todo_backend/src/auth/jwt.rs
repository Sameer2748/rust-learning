use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,   
    pub exp: usize, 
}


pub async fn create_token(user_id:i32) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");
    let claims = Claims {
        sub: user_id,
        exp: (Utc::now() + Duration::hours(24))
            .timestamp() as usize,
    };

     let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    ).unwrap();

    println!("{:?}", token);
    token
}


pub fn verify_token(token: &str) -> Claims {
    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET missing");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .unwrap()
    .claims
}
