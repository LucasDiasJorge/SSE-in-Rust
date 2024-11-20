use actix_web::HttpRequest;
use reqwest::Client;

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ServiceUnavailable,
}

pub fn extract_token(req: &HttpRequest) -> Result<&str, AuthError> {
    req.headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AuthError::MissingToken)
}

pub async fn validate_token(token: &str) -> Result<(), AuthError> {
    let client = Client::new();

    let response = client
        .get("http://localhost:8889/api/v2/core/info")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|_| AuthError::ServiceUnavailable)?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(AuthError::InvalidToken)
    }
}
