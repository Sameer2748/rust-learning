use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono::{Utc, Duration};
use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   
    pub exp: usize, 
}


pub async fn create_token(user_id:Uuid) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET missing");
    let claims = Claims {
        sub: user_id.to_string(),
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


pub fn verify_token(token: &str) -> Uuid {
    let secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET missing");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .unwrap();

    // Convert the string ID back into a UUID
    Uuid::parse_str(&token_data.claims.sub).expect("Invalid UUID in token")
}

