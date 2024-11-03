use actix_web::{get, web, HttpRequest, HttpResponse, Error};
use async_stream::stream;
use bytes::Bytes;
use futures_util::stream::Stream;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::interval;

use crate::services::auth::{extract_token, validate_token, AuthError};
use crate::messages::messages;

#[get("/event/{message}")]
pub async fn sse(req: HttpRequest, info: web::Path<messages::Info>) -> Result<HttpResponse, Error> {
    // Extract and validate the token
    let token = extract_token(&req).map_err(|e| match e {
        AuthError::MissingToken => HttpResponse::Unauthorized().body("Missing authorization token"),
        _ => HttpResponse::InternalServerError().body("Token extraction failed"),
    })?;

    validate_token(token).await.map_err(|e| match e {
        AuthError::InvalidToken => HttpResponse::Unauthorized().body("Invalid token"),
        AuthError::ServiceUnavailable => HttpResponse::InternalServerError().body("Authentication service unreachable"),
        _ => HttpResponse::InternalServerError().body("Token validation failed"),
    })?;

    // Create SSE stream
    let stream = stream! {
        let mut interval = interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            let data = format!("data: {{ \"message\": \"Event {}\" }}\n\n", info.message);
            yield Ok::<_, actix_web::Error>(Bytes::from(data));
        }
    };

    Ok(
        HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(Box::pin(stream) as Pin<Box<dyn Stream<Item = Result<Bytes, actix_web::Error>>>>)
    )
}