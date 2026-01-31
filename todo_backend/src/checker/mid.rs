use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use crate::auth::jwt::verify_token;

pub async fn auth_middleware(
    // we get req mutable so we cna add user id to access further ahead
    mut req: Request,
    next: Next
)->Result<Response , StatusCode>{
    // we get this token and we will use this closure to convert it in a  string
    let auth_token = req.headers().get("Authorization").and_then(|t| t.to_str().ok());
    // add verrfificaiton if it is a valid token like first extract if any bearer if we are not adding that then skip this 
    let token = match auth_token{
        Some(token)=> token,
        _=> return Err(StatusCode::UNAUTHORIZED),
    };
    println!("{}", token);
    // verify token and it will give the id of the user 
    let claims = verify_token(token);
    // we got the user id and we insert that user id in the req
    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)
}