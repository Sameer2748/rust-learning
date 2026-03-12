use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{StatusCode, header},
};
use crate::jwt::jwt::verify_token;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Get the Authorization header
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 2. Check if it's a "Bearer <token>"
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Skip "Bearer "

    // 3. Verify the token using your jwt.rs
    let user_id = verify_token(token);
    // Note: If verify_token panics (because of .unwrap()), we should eventually make it return a Result.

    // 4. Store the User ID in the request so the Handlers can see it!
    req.extensions_mut().insert(user_id);

    // 5. Everything is good, move to the next handler
    Ok(next.run(req).await)
}
