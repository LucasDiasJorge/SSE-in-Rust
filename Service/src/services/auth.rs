use actix_web::HttpRequest;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ServiceUnavailable,
}

// Extract the Bearer token from the authorization header
pub fn extract_token(req: &HttpRequest) -> Result<&str, AuthError> {
    req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AuthError::MissingToken)
}

// Check the token's validity via the external auth service
pub async fn validate_token(token: &str) -> Result<(), AuthError> {
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("token", token.to_string());

    let response = client
        .post("http://localhost:8000/api/v2/token-info")
        .json(&params)
        .send()
        .await
        .map_err(|_| AuthError::ServiceUnavailable)?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(AuthError::InvalidToken)
    }
}
