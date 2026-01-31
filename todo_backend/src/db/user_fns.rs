use sqlx::PgPool;
// mod auth;
use  crate::auth::jwt::{create_token, verify_token};

pub async fn signup_user(pool: &PgPool, name: String, email: String, password: String) -> Result<i32, sqlx::Error> {
    // id is SERIAL PRIMARY KEY, so it auto-generates
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id",
        name,
        email,
        password
    )
    .fetch_one(pool)
    .await?;
    
    Ok(result.id)
}

pub async fn signin_user(pool: &PgPool, email: String, password: String) -> Result<Option<i32>, sqlx::Error> {
    // Signin should SELECT, not INSERT. Check if user exists with matching credentials
    let result = sqlx::query!(
        "SELECT id FROM users WHERE email = $1 AND password = $2",
        email,
        password
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(result.map(|r| r.id))
}