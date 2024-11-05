use std::pin::Pin;
use std::time::Duration;
use actix_web::{Error, HttpRequest, HttpResponse};
use async_stream::stream;
use bytes::Bytes;
use futures_util::Stream;
use tokio::time::interval;
use crate::models::event::EventPayload;
use crate::models::handshake::HandshakePayload;
use crate::services::auth::{extract_token, validate_token, AuthError};

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

pub(crate) fn report_events(message: String) -> Pin<Box<dyn Stream<Item = Result<Bytes, Error>>>> {
    let stream = stream! {
        let mut interval = interval(Duration::from_secs(2));

        let handshake_id = 1;

        let handshake_payload = HandshakePayload {
            id: 1,
            event: String::from("handshake"),
        };

        let handshake_data = format!(
            "id: {}\nevent: {}\n",
            handshake_id,
            handshake_payload.event
        );

        yield Ok::<_, Error>(Bytes::from(handshake_data));

        loop {
            interval.tick().await;

            let payload_event = EventPayload {
                id: -1,
                timestamp: serde_json::Number::from(chrono::Utc::now().timestamp()),
                message: message.clone(),
            };

            let json_payload = serde_json::to_string(&payload_event).unwrap();

            let data = format!("data: {}\n", json_payload);
            yield Ok::<_, Error>(Bytes::from(data));
        }
    };

    Box::pin(stream)
}
