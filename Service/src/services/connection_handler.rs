use std::pin::Pin;
use actix_web::{Error, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures_util::{Stream};
use crate::services::auth::{extract_token, validate_token, AuthError};
use serde_json::{self};
use crate::services::event_handler::{default, report_events, report_pong};

pub(crate) fn route_to_event(event_type: String) -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    if event_type == "realtime" {
        report_events()
    } else if event_type == "ping" {
        report_pong()
    } else {
        default()
    }
}


pub(crate) async fn on_connect(req: &HttpRequest) -> Result<(), HttpResponse> {

    let token = extract_token(req).map_err(|e| match e {
        AuthError::MissingToken => HttpResponse::Unauthorized().body("Missing authorization token"),
        _ => HttpResponse::InternalServerError().body("Token extraction failed"),
    })?;

    validate_token(token).await.map_err(|e| match e {
        AuthError::InvalidToken => HttpResponse::Unauthorized().body("Invalid token"),
        AuthError::ServiceUnavailable => HttpResponse::InternalServerError().body("Authentication service unreachable"),
        _ => HttpResponse::InternalServerError().body("Token validation failed"),
    })?;

    Ok(())
}