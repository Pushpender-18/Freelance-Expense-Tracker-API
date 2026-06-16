use std::time::Duration;

use crate::models::user::Claims;
use axum::{
    body::Body,
    extract::Request,
    http::{
        Response, StatusCode,
        header::{self},
    },
    middleware::Next,
};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

pub async fn verfiy_auth(req: Request, next: Next) -> Result<Response<Body>, StatusCode> {
    match req.headers().get(header::AUTHORIZATION) {
        Some(auth_header) => match auth_header.to_str() {
            Ok(bearer_token) => {
                let json_webtoken: String = bearer_token.chars().skip(7).collect();

                match verify_jwt(&json_webtoken) {
                    Some(_claims) => Ok(next.run(req).await),
                    _ => Err(StatusCode::FORBIDDEN),
                }
            }
            Err(error) => {
                println!("[ERROR] Parsing JWT Token to String: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        None => Err(StatusCode::NON_AUTHORITATIVE_INFORMATION),
    }

    // println!("[INFO] JWT Token: {}", jwt_token);
}

// Generates JWT Token
pub fn generate_jwt(user_id: Uuid) -> Option<String> {
    let now = Utc::now();
    let exp_time = Duration::from_secs(600);
    let claim = Claims::new(user_id, (now + exp_time).timestamp());
    let secret = "To_Be_Written";

    match encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(token) => Some(token),
        Err(error) => {
            println!("[ERROR] JWT Token Generation failed: {}", error);
            None
        }
    }
}

// JWT Token verification
pub fn verify_jwt(token: &str) -> Option<Claims> {
    let secret = "To_Be_Written";

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => {
            let claim = token_data.claims;
            Some(claim)
        }
        Err(error) => {
            println!("[ERROR] JWT Verification Failed: {}", error);
            None
        }
    }
}
